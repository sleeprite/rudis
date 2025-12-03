# Rudis 事务功能说明

Rudis 现在支持 Redis 风格的事务功能，允许将多个命令打包在一起执行。

## 事务命令

- `MULTI` - 开始一个事务
- `EXEC` - 执行事务中的所有命令
- `DISCARD` - 取消事务，清空事务队列

## 使用示例

```
127.0.0.1:6379> MULTI
OK
127.0.0.1:6379> SET key1 value1
QUEUED
127.0.0.1:6379> GET key1
QUEUED
127.0.0.1:6379> EXEC
1) OK
2) "value1"
```

## 事务行为

1. 客户端发送 `MULTI` 命令开始事务
2. 在事务中，除 `EXEC` 和 `DISCARD` 外的所有命令都会被排队，而不是立即执行
3. 客户端发送 `EXEC` 命令执行事务队列中的所有命令
4. 客户端发送 `DISCARD` 命令取消事务，清空队列并退出事务状态

## 错误处理

- 在非事务状态下执行 `EXEC` 或 `DISCARD` 会返回错误
- 事务中的命令错误不会影响其他命令的执行
- 在事务执行过程中不能嵌套使用事务命令（MULTI、EXEC、DISCARD）

## 实现细节

事务功能的实现主要包括以下几个方面：

### 1. Session 状态管理
在 `src/network/session.rs` 中增加了事务相关的字段和方法：
- `in_transaction`: 标记会话是否处于事务模式
- `transaction_frames`: 存储事务队列中的命令帧
- `start_transaction()`: 开始事务
- `is_in_transaction()`: 检查是否在事务中
- `add_transaction_frame()`: 添加命令帧到事务队列
- `get_transaction_frames()`: 获取事务队列中的命令帧
- `clear_transaction()`: 清空事务状态

### 2. 命令解析器扩展
在 `src/command.rs` 中增加了对事务命令的支持：
- 在 Command 枚举中添加了 `Multi`、`Exec` 和 `Discard` 变体
- 在 `parse_from_frame` 方法中增加了对 "MULTI"、"EXEC" 和 "DISCARD" 命令的解析

### 3. 服务器处理逻辑
在 `src/server.rs` 中增加了对事务模式的支持：
- 修改了 `Handler::handle` 方法，在事务模式下对命令进行排队
- 添加了 `execute_transaction` 方法来执行事务队列中的所有命令

### 4. 事务命令实现
在 `src/cmds/transaction/` 目录下实现了三个事务命令：
- `multi.rs`: 实现 MULTI 命令，用于开始事务
- `exec.rs`: 实现 EXEC 命令，用于执行事务队列
- `discard.rs`: 实现 DISCARD 命令，用于取消事务

## 测试

事务功能的测试位于 `tests/test_transactions.rs` 文件中，包括：
- `test_basic_transaction`: 测试事务的基本功能
- `test_discard_transaction`: 测试 DISCARD 命令的功能
- `test_exec_discard_without_multi`: 测试在非事务模式下使用 EXEC 和 DISCARD 命令的情况