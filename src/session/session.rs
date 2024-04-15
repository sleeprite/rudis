/*
 * 会话信息
 * 
 * @param selected_database 索引, 默认: 0
 * @param authenticated 认证, 默认：false
 */
pub struct Session {
    selected_database: usize,
    authenticated: bool,
}

impl Session {
    
    pub fn new() -> Session {
        Session {
            selected_database: 0,
            authenticated: false,
        }
    }

    pub fn get_selected_database(&self) -> usize {
        self.selected_database
    }

    pub fn set_selected_database(&mut self, index: usize) {
        self.selected_database = index;
    }

    pub fn get_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn set_authenticated(&mut self, status: bool) {
        self.authenticated = status;
    }
}