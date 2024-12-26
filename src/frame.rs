/*
 * 命令帧枚举
 */
pub enum Frame {
    Ok,
    Integer(i64),
    SimpleString(String),
    Array(Vec<String>),
    BulkString(Option<String>),
    Error(String),
    Null
}

impl Frame {

    /**
     * 将 frame 转换为 bytes
     * 
     * @param self 本身
     */
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Frame::Ok => b"+OK\r\n".to_vec(),
            Frame::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            Frame::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Frame::BulkString(None) => b"$-1\r\n".to_vec(),
            Frame::Null => b"$-1\r\n".to_vec(),
            Frame::Error(e) => format!("-{}\r\n", e).into_bytes(),
            Frame::Array(arr) => {
                let mut bytes = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    bytes.extend(format!("${}\r\n{}\r\n", item.len(), item).into_bytes());
                }
                bytes
            },
            Frame::BulkString(Some(s)) => {
                let mut bytes = format!("${}\r\n", s.len()).into_bytes();
                bytes.extend(s.as_bytes());
                bytes.extend(b"\r\n");
                bytes
            },
        }
    }
    
    /**
     * 通过解析 bytes 创建命令帧
     *
     * @param bytes 二进制
     */
    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Frame, Box<dyn std::error::Error>> {
        match bytes[0] {
            b'+' => Frame::parse_simple_string(bytes),
            b'*' => Frame::parse_array(bytes),
            _ => Err("Unknown frame type".into()),
        }
    }

    /**
     * 简单字符串
     *
     * @param bytes 二进制
     */
    fn parse_simple_string(bytes: &[u8]) -> Result<Frame, Box<dyn std::error::Error>> {
        let end = bytes.iter().position(|&x| x == b'\r').unwrap();
        let content = String::from_utf8(bytes[1..end].to_vec())?;
        Ok(Frame::SimpleString(content))
    }

    /**
     * 数组字符串
     *
     * @param bytes 二进制
     */
    fn parse_array(bytes: &[u8]) -> Result<Frame, Box<dyn std::error::Error>> {
        let mut frames = Vec::new();
        let mut start = 0;
        for (i, &item) in bytes.iter().enumerate() {
            if item == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                let part = match std::str::from_utf8(&bytes[start..i]) {
                    Ok(v) => v,
                    Err(_) => return Err("Invalid UTF-8 sequence".into()),
                };

                if !(part.starts_with('*') || part.starts_with('$')) {
                    frames.push(part.to_string());
                }

                start = i + 2;
            }
        }
        Ok(Frame::Array(frames))
    }

    /**
     * 获取指定索引的内容
     *
     * @param index 索引
     */
    pub fn get_arg(&self, index: usize) -> Option<&String> {
        match self {
            Frame::Array(array) => {
                if index < array.len() {
                    Some(&array[index])
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /**
     * 获取命令帧中的所有参数
     * 
     * @param self 本身
     * 
     * @return 一个包含所有参数的字符串向量，如果不是 Array 类型则返回空向量
     */
    pub fn get_args(&self) -> Vec<String> {
        match self {
            Frame::Array(array) => array.clone(),
            _ => Vec::new(),
        }
    }

    /**
     * 获取从指定索引开始的内容集合
     * 
     * @param self 本身
     * @param start_index 开始索引
     * 
     * @return 一个包含从指定索引开始的所有参数的字符串向量，如果不是 Array 类型或索引超出范围则返回空向量
     */
    pub fn get_args_from_index(&self, start_index: usize) -> Vec<String> {
        match self {
            Frame::Array(array) => {
                if start_index < array.len() {
                    array[start_index..].to_vec()
                } else {
                    Vec::new() // 响应空数组
                }
            },
            _ => Vec::new()
        }
    }
}