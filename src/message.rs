use tokio::sync::oneshot;

use crate::{command::Command, frame::Frame};

/**
 * 消息
 * 
 * @param sender 发送者
 * @param command 命令
 */
pub struct Message {
    pub sender: oneshot::Sender<Frame>,
    pub command: Command,
}