use crate::persistence::rdb_file::RdbFile;

/*
 * 命令帧枚举
 */
#[derive(Clone)]
pub enum Frame {
    Ok,
    Integer(i64),
    RDBFile(Vec<u8>),
    SimpleString(String),
    Array(Vec<Frame>),
    BulkString(String),
    Error(String),
    Null
}

impl Frame {

    /**
     * 将 frame 转化为字符串
     * 
     * @param self 本身
     */
    pub fn to_string(&self) -> String {
        match self {
            Frame::Ok => String::from("OK"),
            Frame::Integer(i) => i.to_string(),
            Frame::RDBFile(data) => format!("[RDBFile {} bytes]", data.len()),
            Frame::SimpleString(s) => s.clone(),
            Frame::BulkString(s) => s.clone(),
            Frame::Error(e) => e.clone(),
            Frame::Null => String::new(),
            Frame::Array(arr) => {
                let mut result = String::new();
                for item in arr {
                    result.push_str(&item.to_string());
                    result.push(' ');
                }
                result.trim_end().to_string()
            },
        }
    }

    /**
     * 将 frame 转换为 bytes
     * 
     * @param self 本身
     */
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Frame::Ok => b"+OK\r\n".to_vec(),
            Frame::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Frame::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            Frame::Error(e) => format!("-{}\r\n", e).into_bytes(),
            Frame::Null => b"$-1\r\n".to_vec(),
            Frame::RDBFile(data) => {
                let mut bytes = format!("~{}\r\n", data.len()).into_bytes();
                bytes.extend(data);
                bytes.extend(b"\r\n");
                bytes
            },
            Frame::Array(arr) => {
                let mut bytes = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    bytes.extend(item.as_bytes());
                }
                bytes
            },
            Frame::BulkString(s) => {
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
            b'~' => Frame::parse_rdb_file(bytes),
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
     * 正确解析 RDB 文件帧
     * 
     * @param bytes 二进制
     */
    fn parse_rdb_file(bytes: &[u8]) -> Result<Frame, Box<dyn std::error::Error>> {

        let mut len_end = None;
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'\r' {
                len_end = Some(i);
                break;
            }
        }

        let len_end = match len_end {
            Some(pos) => pos,
            None => return Err("Invalid RDB format: missing CR".into()),
        };

        let len_bytes = &bytes[1..len_end];
        let len_str = match std::str::from_utf8(len_bytes) {
            Ok(s) => s,
            Err(e) => return Err(format!("Invalid UTF-8: {}", e).into()),
        };

        let data_len = match len_str.parse::<usize>() {
            Ok(n) => n,
            Err(e) => return Err(format!("Invalid RDB length: {} ({})", len_str, e).into()),
        };

        let data_start = len_end + 2;
        let data_end = data_start + data_len;

        if bytes.len() < data_end + 2 {
            return Err(format!("RDB data incomplete: expected {} bytes, got {}", data_end + 2, bytes.len()).into());
        }

        if bytes[data_end] != b'\r' || bytes[data_end + 1] != b'\n' {
            return Err("Invalid RDB terminator".into());
        }

        let mut data = Vec::with_capacity(data_len);
        for &byte in &bytes[data_start..data_end] {
            data.push(byte);
        }

        Ok(Frame::RDBFile(data))
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

                // 处理 keys * 命令
                if !((part.starts_with('*') && part.len()!= 1) || part.starts_with('$')) {
                    frames.push(Frame::SimpleString(part.to_string()));
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
    pub fn get_arg(&self, index: usize) -> Option<String> {
        match self {
            Frame::Array(array) => {
                if index < array.len() {
                    Some(array[index].to_string())
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
            Frame::Array(array) => array.iter().map(|frame| frame.to_string()).collect(),
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
                    array[start_index..].iter().map(|frame| frame.to_string()).collect()
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
        }
    }

    /**
     * 将 RDBFile 帧转换为 RdbFile 对象
     * 
     * @return Result<RdbFile, Box<dyn std::error::Error>> 转换结果
     */
    pub fn to_rdb_file(&self) -> Result<RdbFile, Box<dyn std::error::Error>> {
        match self {
            Frame::RDBFile(data) => {
                RdbFile::from_bytes(data).map_err(|e| format!("Failed to parse RDB file: {}", e).into())
            }
            _ => Err("Frame is not an RDBFile".into()),
        }
    }
}