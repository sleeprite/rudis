use anyhow::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub struct Connection {
    stream: TcpStream
}

impl Connection {

    pub fn new(stream: TcpStream) ->  Self {
        return Connection {
            stream
        }
    }

    pub async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut temp_bytes: [u8; 1024] = [0; 1024]; 
        loop {
            let n = match self.stream.read(&mut temp_bytes).await {
                Ok(n) => n,
                Err(e) => {
                    return Err(Error::msg(format!("Failed to read from stream: {:?}", e)));
                }
            };

            if n == 0 {
                if bytes.is_empty() {
                    // 连接关闭且未读取到任何数据
                    return Err(Error::msg("Connection closed by peer"));
                } else {
                    // 连接关闭但已读取部分数据
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

    pub async fn write_bytes(&mut self, bytes: Vec<u8>) {
        if let Err(e) = self.stream.write_all(&bytes).await {
            eprintln!("Failed to write to socket; err = {:?}", e);
        }
    }
}