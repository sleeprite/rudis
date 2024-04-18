pub enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<RespValue>),
}

impl RespValue {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RespValue::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            RespValue::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespValue::Error(e) => format!("-{}\r\n", e).into_bytes(),
            RespValue::BulkString(s) => {
                if let Some(s) = s {
                    format!("${}\r\n{}\r\n", s.len(), s).into_bytes()
                } else {
                    "$-1\r\n".to_string().into_bytes()
                }
            }
            RespValue::Array(values) => {
                let mut result = format!("*{}\r\n", values.len()).into_bytes();
                for value in values {
                    result.extend_from_slice(&value.to_bytes());
                }
                result
            }
        }
    }
}