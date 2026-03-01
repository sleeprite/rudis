# Rudis 代码问题汇总

## 1. SRANDMEMBER 命令 - 参数数量检查不完整

**文件**: `src/cmds/set/srandmember.rs` (第 12-18 行)

**问题描述**: 只检查了 key 是否存在，但没有检查参数数量是否超过 3 个。如果传入超过 3 个参数，应该报错。

**当前代码**:
```rust
pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
    let key = frame.get_arg(1);
    if key.is_none() {
        return Err(Error::msg("ERR wrong number of arguments for 'srandmember' command"));
    }
    // ...
}
```

**修复建议**:
```rust
pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
    let args = frame.get_args();
    if args.len() < 2 || args.len() > 3 {
        return Err(Error::msg("ERR wrong number of arguments for 'srandmember' command"));
    }
    // ...
}
```

---

## 2. DB::ttl_millis 返回值逻辑错误

**文件**: `src/store/db.rs` (第 364-382 行)

**问题描述**: 当 key 已过期并被删除后，函数返回 `-1`，但按照 Redis 协议，应该返回 `-2`（key 不存在）。

**当前代码**:
```rust
pub fn ttl_millis(&mut self, key: &str) -> i64 {
    if let Some(expire_time) = self.expire_records.get(key) {
        let now = SystemTime::now();
        if now >= *expire_time {
            self.remove(key);
            -1  // BUG: 这里应该返回 -2
        } else {
            // ...
        }
    }
    // ...
}
```

**修复建议**: 将过期删除后的返回值从 `-1` 改为 `-2`。

---

## 3. Frame::parse_simple_string 潜在 panic

**文件**: `src/frame.rs` (第 299-303 行)

**问题描述**: 如果输入的字节数组中没有 `\r`，`position()` 返回 `None`，`unwrap()` 会导致 panic。

**当前代码**:
```rust
fn parse_simple_string(bytes: &[u8]) -> Result<Frame, Error> {
    let end = bytes.iter().position(|&x| x == b'\r').unwrap();  // 可能 panic
    let content = String::from_utf8(bytes[1..end].to_vec())?;
    Ok(Frame::SimpleString(content))
}
```

**修复建议**: 使用 `ok_or_else()` 替代 `unwrap()`，返回合适的错误信息。

---

## 4. Connection::read_bytes 逻辑缺陷

**文件**: `src/network/connection.rs` (第 19-44 行)

**问题描述**: 使用 `n < temp_bytes.len()` 来判断数据是否读取完整是不正确的。网络数据可能分多次到达，即使本次读取的字节数小于缓冲区大小，也不代表所有数据都已接收完毕。这可能导致命令解析失败。

**当前代码**:
```rust
pub async fn read_bytes(&self) -> Result<Vec<u8>, Error> {
    // ...
    loop {
        let n = stream.read(&mut temp_bytes).await?;
        if n == 0 {
            if bytes.is_empty() {
                return Err(Error::msg("Connection closed by peer"));
            } else {
                break;  // 可能数据不完整就退出
            }
        }
        bytes.extend_from_slice(&temp_bytes[..n]);
        if n < temp_bytes.len() {  // 不能保证数据完整
            break;
        }
    }
    Ok(bytes)
}
```

**修复建议**: 需要实现基于 Redis 协议的数据帧边界检测，而不是依赖缓冲区大小判断。

---

## 5. AofFile::read_all_frames 分隔符处理问题

**文件**: `src/persistence/aof_file.rs` (第 49-72 行)

**问题描述**: AOF 文件使用 `\r\n\r\n` 作为帧分隔符，但每个帧本身也以 `\r\n` 结尾。这种设计可能导致解析歧义。另外，如果文件末尾没有分隔符，最后一个帧会被忽略。

**当前代码**:
```rust
pub async fn read_all_frames(&self) -> Result<Vec<Frame>> {
    // ...
    let separator = b"\r\n\r\n";
    
    while let Some(pos) = content[start..].windows(separator.len()).position(|window| window == separator) {
        let end = start + pos;
        let frame_data = &content[start..end + separator.len() / 2];
        // ...
    }
}
```

---

## 6. 命令解析缺少对空参数的检查

**文件**: `src/command.rs` (第 156-157 行)

**问题描述**: 如果 frame 是空数组，`get_arg(0)` 返回 `None`，`unwrap()` 会导致 panic。

**当前代码**:
```rust
pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
    let command_name = frame.get_arg(0).unwrap();  // 可能 panic
    // ...
}
```

**修复建议**: 使用 `ok_or_else()` 替代 `unwrap()`。

---

## 7. Server::replay_aof_file 中 SELECT 命令处理不完整

**文件**: `src/server.rs` (第 145-159 行)

**问题描述**: AOF 重放时，SELECT 命令只更新了本地变量 `current_db_index`，但没有将这个 SELECT 命令发送到数据库执行。如果 AOF 文件中有跨数据库的操作，重放后数据库状态可能不正确。

**当前代码**:
```rust
Command::Select(select) => {
    current_db_index = select.get_db_index();
},
_ => {
    let db_sender = db_manager.get_sender(current_db_index);
    // ...
}
```

---

## 8. SPOP 命令随机性不足

**文件**: `src/cmds/set/spop.rs` (第 42-48 行)

**问题描述**: `set.iter().next()` 总是返回集合中的第一个元素（按照 HashSet 的内部顺序），而不是随机元素。这与 Redis 的 SPOP 命令语义不符（Redis 的 SPOP 应该随机移除元素）。

**当前代码**:
```rust
for _ in 0..pop_count {
    if let Some(member) = set.iter().next().cloned() {  // 总是取第一个元素
        set.remove(&member);
        popped_members.push(Frame::BulkString(member));
    } else {
        break;
    }
}
```

**修复建议**: 使用随机数生成器选择随机元素，参考 SRANDMEMBER 的实现。

---

## 问题严重程度分级

| 级别 | 问题编号 | 说明 |
|------|----------|------|
| 高 | 3, 4, 6 | 可能导致程序 panic |
| 中 | 1, 2, 7, 8 | 功能或协议不兼容 |
| 低 | 5 | 边界情况处理不完善 |
