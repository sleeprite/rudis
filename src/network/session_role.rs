/// 会话角色标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRole {
    /// 从节点
    Slave,
    /// 其他角色（可根据需要扩展）
    Other,
}

impl Default for SessionRole {
    fn default() -> Self {
        SessionRole::Other
    }
}

impl SessionRole {
    
    /// 检查是否为从节点
    pub fn is_slave(&self) -> bool {
        matches!(self, SessionRole::Slave)
    }
}