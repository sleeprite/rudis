use anyhow::Error;
use crate::command::Command;
use crate::frame::Frame;
use crate::server::Handler;
use crate::server::blocking::{BlockDirection, BlockingQueueManager};

/// 统一的异步命令分发入口
///
/// **设计原则**：
/// - 只处理需要 Handler 上下文的异步命令（如 BLPOP/BRPOP）
/// - 所有"哪些命令需要 Handler"的判断都集中在这里
/// - 如果命令不需要 Handler，返回 None，让调用者按普通命令处理
pub async fn dispatch(
    handler: &mut Handler,
    command: &Command,
) -> Option<Result<Frame, Error>> {
    match command {
        // 阻塞命令
        Command::Blpop(blpop) => Some(blpop.clone().apply(handler).await),
        Command::Brpop(brpop) => Some(brpop.clone().apply(handler).await),
        Command::Lpush(lpush) => Some(handle_blocking_aware(handler, Command::Lpush(lpush.clone())).await),
        Command::Rpush(rpush) => Some(handle_blocking_aware(handler, Command::Rpush(rpush.clone())).await),
        // 其他命令：不在这里处理，返回 None 让调用者按普通命令处理
        _ => None,
    }
}

/// 统一处理需要阻塞检查的命令（当前是 LPUSH/RPUSH）
///
/// 如果命令有阻塞等待者，直接唤醒并转交数据（不存数据库）
/// 否则正常执行数据库操作
///
/// 这样设计的好处：
/// 1. Handler 保持简洁，不需要为每个命令添加 handle_xxx 方法
/// 2. 阻塞逻辑集中在 async_dispatch 模块，不再依赖外部 blocking 模块
/// 3. 易于扩展：只需在 try_wakeup 中添加新命令的处理
async fn handle_blocking_aware(
    handler: &mut Handler,
    command: Command,
) -> Result<Frame, Error> {

    // 尝试唤醒阻塞的客户端
    let wakeup_result = {
        let mut blocking_manager = handler.get_state().blocking_list.lock().await;
        try_wakeup(&command, &mut blocking_manager)
    };

    if let Some((session_id, response_frame)) = wakeup_result {
        // 找到等待的 session，发送响应
        if let Some(session) = handler.get_session_manager().get_session(session_id) {
            session.connection.write_bytes(response_frame.as_bytes()).await;
        }

        // 返回成功（但不存数据库）
        return Ok(Frame::Integer(1));
    }

    // 没有等待者或唤醒失败，正常执行数据库操作
    handler.apply_db_command(command).await
}

/// 尝试为命令唤醒阻塞的客户端
/// 
/// 如果命令是 LPUSH/RPUSH 且有关键的阻塞等待者，直接唤醒并返回结果
/// 否则返回 None，表示需要正常执行数据库操作
///
fn try_wakeup(
    command: &Command,
    blocking_manager: &mut BlockingQueueManager,
) -> Option<(usize, Frame)> {
    match command {
        Command::Lpush(lpush) => {
            // 遍历所有值，尝试唤醒多个等待者
            for value in lpush.values() {
                if let Some((session_id, response_frame)) = blocking_manager.try_wakeup(
                    lpush.key(),
                    BlockDirection::Left,
                    value.clone(),
                ) {
                    return Some((session_id, response_frame));
                }
            }
            None
        },
        Command::Rpush(rpush) => {
            // 遍历所有值，尝试唤醒多个等待者
            for value in rpush.values() {
                if let Some((session_id, response_frame)) = blocking_manager.try_wakeup(
                    rpush.key(),
                    BlockDirection::Right,
                    value.clone(),
                ) {
                    return Some((session_id, response_frame));
                }
            }
            None
        },
        _ => None,
    }
}
