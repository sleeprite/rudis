pub enum RespValue {
    SimpleString(String),
    Integer(i64),
    Error(String)
}

impl RespValue {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            RespValue::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            RespValue::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespValue::Error(e) => format!("-{}\r\n", e).into_bytes()
        }
    }
}