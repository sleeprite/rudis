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
        let mut bytes = Vec::new();
        let mut temp_bytes = [0; 1024];
        loop {
            let n = match self.stream.read(&mut temp_bytes).await {
                Ok(n) => n,
                Err(e) => {  
                    return Err(Error::msg(format!("Failed to read from stream: {:?}", e)));
                }
            };
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