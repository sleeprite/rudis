pub enum RespValue {
    SimpleString(String),
    Integer(i64),
    Error(String),
    Null,
    BulkString(String),
    Ok
}

impl RespValue {
    
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            RespValue::Ok => b"+OK\r\n".to_vec(),
            RespValue::Null => b"$-1\r\n".to_vec(),
            RespValue::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            RespValue::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespValue::Error(e) => format!("-{}\r\n", e).into_bytes(),
            RespValue::BulkString(s) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(b"$");
                bytes.extend_from_slice(&(s.len() as i64).to_string().into_bytes());
                bytes.extend_from_slice(b"\r\n");
                bytes.extend_from_slice(s.as_bytes());
                bytes.extend_from_slice(b"\r\n");
                bytes
            },
        }
    }
}