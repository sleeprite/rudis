#[cfg(test)]
mod tests {
    use rudis_server::frame::Frame;
    use anyhow::Error;

    #[test]
    fn test_parse_multiple_frames() -> Result<(), Error> {
        // 模拟redis-rust客户端发送的粘连命令
        // CLIENT SETINFO LIB-NAME redis-rs
        // CLIENT SETINFO LIB-VER 1.0.0-rc.4
        let bytes = b"*3\r\n$6\r\nCLIENT\r\n$7\r\nSETINFO\r\n$8\r\nLIB-NAME\r\n*3\r\n$6\r\nCLIENT\r\n$7\r\nSETINFO\r\n$8\r\nLIB-VER\r\n";
        
        let frames = Frame::parse_multiple_frames(bytes)?;
        
        assert_eq!(frames.len(), 2);
        
        // 验证第一个命令
        let first_frame = &frames[0];
        let args1 = first_frame.get_args();
        assert_eq!(args1.len(), 3);
        assert_eq!(args1[0], "CLIENT");
        assert_eq!(args1[1], "SETINFO");
        assert_eq!(args1[2], "LIB-NAME");
        
        // 验证第二个命令
        let second_frame = &frames[1];
        let args2 = second_frame.get_args();
        assert_eq!(args2.len(), 3);
        assert_eq!(args2[0], "CLIENT");
        assert_eq!(args2[1], "SETINFO");
        assert_eq!(args2[2], "LIB-VER");
        
        Ok(())
    }
}