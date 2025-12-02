# Rudis 粘连命令处理机制详解

## 问题背景

在使用 `redis-rust` 客户端库连接到 rudis 服务器时，该库会在建立连接后立即连续发送两个 CLIENT 命令：

```
CLIENT SETINFO LIB-NAME redis-rs
CLIENT SETINFO LIB-VER 1.0.0-rc.4
```

这两个命令可能会在同一个 TCP 数据包中发送到服务器，形成所谓的"粘连命令"。原始实现无法正确处理这种情况，导致客户端在等待第二个响应时超时。

## 调整前的代码逻辑

### 1. 网络数据读取 (connection.rs)

```rust
pub async fn read_bytes(&self) -> Result<Vec<u8>, Error> {
    let mut stream = self.stream.lock().await;
    let mut bytes: Vec<u8> = Vec::new();
    let mut temp_bytes: [u8; 1024] = [0; 1024]; 
    
    loop {
        let n = match stream.read(&mut temp_bytes).await {
            Ok(n) => n,
            Err(e) => {
                return Err(Error::msg(format!("Failed to read from stream: {:?}", e)));
            }
        };

        if n == 0 {
            if bytes.is_empty() {
                return Err(Error::msg("Connection closed by peer"));
            } else {
                break;
            }
        }
        bytes.extend_from_slice(&temp_bytes[..n]);
        if n < temp_bytes.len() {
            break;
        }
    }
    Ok(bytes)
}
```

这个方法会读取 TCP 流中的所有可用数据，可能包含多个命令。

### 2. 帧解析 (frame.rs)

```rust
pub fn parse_from_bytes(bytes: &[u8]) -> Result<Frame, Error> {
    match bytes[0] {
        b'+' => Frame::parse_simple_string(bytes),
        b'~' => Frame::parse_rdb_file(bytes),
        b'*' => Frame::parse_array(bytes),  // 只处理第一个命令
        _ => Err(Error::msg("Unknown frame type")),
    }
}
```

原始的解析方法只会解析第一个完整的命令帧，忽略后续可能存在的其他命令。

### 3. 服务器处理逻辑 (server.rs)

```rust
let frame = Frame::parse_from_bytes(bytes.as_slice()).unwrap();
// 只处理一个命令帧
let command = Command::parse_from_frame(frame)?;
// 执行命令并发送一次响应
self.session.connection.write_bytes(frame.as_bytes()).await;
```

服务器只能处理第一个命令，对后续的粘连命令完全无视，导致客户端在等待第二个响应时超时。

## 调整后的代码逻辑

### 1. 新增的多帧解析功能 (frame.rs)

我们添加了三个新的方法来处理粘连命令：

```rust
/// 解析粘连的多个命令帧
pub fn parse_multiple_frames(bytes: &[u8]) -> Result<Vec<Frame>, Error> {
    let mut frames = Vec::new();
    let mut position = 0;
    
    while position < bytes.len() {
        // 查找下一个完整的命令帧
        if let Some(frame_end) = Frame::find_frame_end(&bytes[position..]) {
            let frame_bytes = &bytes[position..position + frame_end];
            let frame = Frame::parse_from_bytes(frame_bytes)?;
            frames.push(frame);
            position += frame_end;
        } else {
            // 如果找不到完整的帧结束位置，将剩余的数据作为最后一个帧处理
            let frame_bytes = &bytes[position..];
            let frame = Frame::parse_from_bytes(frame_bytes)?;
            frames.push(frame);
            break;
        }
    }
    
    Ok(frames)
}

/// 查找单个命令帧的结束位置
fn find_frame_end(bytes: &[u8]) -> Option<usize> {
    // 根据Redis协议规范，准确计算每个命令帧的边界
    // 支持 *数组、+简单字符串、-错误、:整数、$批量字符串、~RDB文件等类型
}

/// 查找元素的结束位置
fn find_element_end(bytes: &[u8]) -> Option<usize> {
    // 用于递归查找数组中嵌套元素的边界
}
```

### 2. 修改后的服务器处理逻辑 (server.rs)

```rust
// 解析可能的多个粘连命令帧
let frames = match Frame::parse_multiple_frames(bytes.as_slice()) {
    Ok(frames) => frames,
    Err(e) => {
        log::error!("Failed to parse multiple frames: {:?}", e);
        let frame = Frame::Error(format!("Failed to parse frames: {:?}", e));
        self.session.connection.write_bytes(frame.as_bytes()).await;
        continue;
    }
};

log::debug!("Received bytes: {:?}", String::from_utf8_lossy(bytes.as_slice()));

// 处理每个帧
for frame in frames {
    log::debug!("Received frame: {}", frame.to_string());
    let frame_copy = frame.clone();
    let command = match Command::parse_from_frame(frame) {
        Ok(cmd) => cmd,
        Err(e) => {
            let frame = Frame::Error(e.to_string());
            self.session.connection.write_bytes(frame.as_bytes()).await;
            continue;
        }
    };
    
    // ... 原有的命令处理逻辑 ...
    
    // 为每个命令发送对应的响应
    self.session.connection.write_bytes(frame.as_bytes()).await;
}
```

## 关键改进点

1. **完整的帧边界识别**：新增的 `find_frame_end` 方法能够准确识别各种 Redis 协议帧的边界，确保不会将多个命令错误地合并为一个。

2. **逐个处理粘连命令**：服务器现在会循环处理所有解析出的命令帧，而不是只处理第一个。

3. **完整的响应机制**：对于每个接收到的命令，都会发送相应的响应，确保客户端能收到预期的回复。

4. **错误处理增强**：增加了对多帧解析过程中可能出现的错误的处理，提高了系统的健壮性。

## 测试验证

我们创建的测试用例验证了该实现能够正确解析以下粘连命令：

```
*3\r
$6\r
CLIENT\r
$7\r
SETINFO\r
$8\r
LIB-NAME\r
*3\r
$6\r
CLIENT\r
$7\r
SETINFO\r
$8\r
LIB-VER\r

```

测试确认能够正确分离出两个独立的命令帧，并且每个帧都能正确解析出其参数。

这个修复确保了 rudis 服务器能够正确处理来自 redis-rust 客户端或其他可能发送粘连命令的客户端的请求，避免了因响应不匹配导致的连接超时问题。