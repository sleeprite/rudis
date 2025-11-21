// src/network/connection.rs
use anyhow::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Connection {
    stream: Arc<Mutex<TcpStream>>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

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

    pub async fn write_bytes(&self, bytes: Vec<u8>) {
        let mut stream = self.stream.lock().await;
        if let Err(e) = stream.write_all(&bytes).await {
            eprintln!("Failed to write to socket; err = {:?}", e);
        }
    }
}