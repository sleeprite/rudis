use crate::persistence::rdb_file::RdbFile;
use anyhow::Error;

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
    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Frame, Error> {
        match bytes[0] {
            b'+' => Frame::parse_simple_string(bytes),
            b'~' => Frame::parse_rdb_file(bytes),
            b'*' => Frame::parse_array(bytes),
            _ => Err(Error::msg("Unknown frame type")),
        }
    }

    /**
     * 解析粘连的多个命令帧
     *
     * @param bytes 二进制数据
     */
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
    
    /**
     * 查找单个命令帧的结束位置
     *
     * @param bytes 二进制数据
     */
    fn find_frame_end(bytes: &[u8]) -> Option<usize> {
       
        if bytes.is_empty() {
            return None;
        }
        
        match bytes[0] {
            b'*' => {
                // 数组类型的帧
                // 首先找到数组长度行的结束位置
                let mut line_end = None;
                for i in 1..bytes.len().min(20) { // 限制搜索范围，防止过长的第一行
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        line_end = Some(i + 2);
                        break;
                    }
                }
                
                let line_end = line_end?;
                if line_end >= bytes.len() {
                    return None;
                }
                
                // 解析数组长度
                let line = std::str::from_utf8(&bytes[1..line_end - 2]).ok()?;
                let array_len: usize = line.parse().ok()?;
                
                // 计算数组元素的结束位置
                let mut current_pos = line_end;
                for _ in 0..array_len {
                    if current_pos >= bytes.len() {
                        return None;
                    }
                    
                    // 查找当前元素的结束位置
                    if let Some(element_end) = Frame::find_element_end(&bytes[current_pos..]) {
                        current_pos += element_end;
                    } else {
                        return None;
                    }
                }
                
                Some(current_pos)
            },
            b'+' | b'-' | b':' => {
                // 简单字符串、错误、整数类型
                for i in 1..bytes.len() {
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        return Some(i + 2);
                    }
                }
                None
            },
            b'$' => {
                // 批量字符串类型
                // 找到长度行的结束
                let mut line_end = None;
                for i in 1..bytes.len().min(20) {
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        line_end = Some(i + 2);
                        break;
                    }
                }
                
                let line_end = line_end?;
                if line_end >= bytes.len() {
                    return None;
                }
                
                // 解析字符串长度
                let line = std::str::from_utf8(&bytes[1..line_end - 2]).ok()?;
                if line == "-1" {
                    // NULL批量字符串
                    return Some(line_end);
                }
                
                let str_len: usize = line.parse().ok()?;
                
                // 字符串内容 + \r\n
                if line_end + str_len + 2 <= bytes.len() {
                    Some(line_end + str_len + 2)
                } else {
                    None
                }
            },
            b'~' => {
                // RDB文件类型
                let mut len_end = None;
                for i in 1..bytes.len().min(20) {
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        len_end = Some(i + 2);
                        break;
                    }
                }
                
                let len_end = len_end?;
                if len_end >= bytes.len() {
                    return None;
                }
                
                let len_str = std::str::from_utf8(&bytes[1..len_end - 2]).ok()?;
                let data_len: usize = len_str.parse().ok()?;
                
                if len_end + data_len + 2 <= bytes.len() {
                    Some(len_end + data_len + 2)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
    
    /**
     * 查找元素的结束位置
     *
     * @param bytes 二进制数据
     */
    fn find_element_end(bytes: &[u8]) -> Option<usize> {
        if bytes.is_empty() {
            return None;
        }
        
        match bytes[0] {
            b'*' => Frame::find_frame_end(bytes),
            b'+' | b'-' | b':' => {
                for i in 1..bytes.len() {
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        return Some(i + 2);
                    }
                }
                None
            },
            b'$' => {
                // 找到长度行的结束
                let mut line_end = None;
                for i in 1..bytes.len().min(20) {
                    if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        line_end = Some(i + 2);
                        break;
                    }
                }
                
                let line_end = line_end?;
                if line_end >= bytes.len() {
                    return None;
                }
                
                // 解析字符串长度
                let line = std::str::from_utf8(&bytes[1..line_end - 2]).ok()?;
                if line == "-1" {
                    // NULL批量字符串
                    return Some(line_end);
                }
                
                let str_len: usize = line.parse().ok()?;
                
                // 字符串内容 + \r\n
                if line_end + str_len + 2 <= bytes.len() {
                    Some(line_end + str_len + 2)
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /**
     * 简单字符串
     *
     * @param bytes 二进制
     */
    fn parse_simple_string(bytes: &[u8]) -> Result<Frame, Error> {
        let end = bytes.iter().position(|&x| x == b'\r').unwrap();
        let content = String::from_utf8(bytes[1..end].to_vec())?;
        Ok(Frame::SimpleString(content))
    }

    /**
     * 正确解析 RDB 文件帧
     * 
     * @param bytes 二进制
     */
    fn parse_rdb_file(bytes: &[u8]) -> Result<Frame, Error> {

        let mut len_end = None;
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'\r' {
                len_end = Some(i);
                break;
            }
        }

        let len_end = match len_end {
            Some(pos) => pos,
            None => return Err(Error::msg("Invalid RDB format: missing CR")),
        };

        let len_bytes = &bytes[1..len_end];
        let len_str = match std::str::from_utf8(len_bytes) {
            Ok(s) => s,
            Err(e) => return Err(Error::msg(format!("Invalid UTF-8: {}", e))),
        };

        let data_len = match len_str.parse::<usize>() {
            Ok(n) => n,
            Err(e) => return Err(Error::msg(format!("Invalid RDB length: {} ({})", len_str, e))),
        };

        let data_start = len_end + 2;
        let data_end = data_start + data_len;

        if bytes.len() < data_end + 2 {
            return Err(Error::msg(format!("RDB data incomplete: expected {} bytes, got {}", data_end + 2, bytes.len())));
        }

        if bytes[data_end] != b'\r' || bytes[data_end + 1] != b'\n' {
            return Err(Error::msg("Invalid RDB terminator"));
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
    fn  parse_array(bytes: &[u8]) -> Result<Frame, Error> {
        let mut frames = Vec::new();
        let mut start = 0;

        for (i, &item) in bytes.iter().enumerate() {

            if item == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {

                let part = match std::str::from_utf8(&bytes[start..i]) {
                    Ok(v) => v,
                    Err(_) => return Err(Error::msg("Invalid UTF-8 sequence")),
                };

                if !((part.starts_with('*') && part.len()!= 1) || part.starts_with('$')) {
                    frames.push(Frame::BulkString(part.to_string()));
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
     * @return Result<RdbFile, Error> 转换结果
     */
    pub fn to_rdb_file(&self) -> Result<RdbFile, Error> {
        match self {
            Frame::RDBFile(data) => {
                RdbFile::from_bytes(data).map_err(|e| Error::msg(format!("Failed to parse RDB file: {}", e)))
            }
            _ => Err(Error::msg("Frame is not an RDBFile")),
        }
    }
}