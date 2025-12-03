use anyhow::Error;
use crate::{frame::Frame, server::Handler};

pub struct Move {
    key: String,
    db_index: usize,
}

impl Move {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        
        if args.len() != 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'move' command"));
        }
        
        let key = args[1].to_string();
        let db_index = match args[2].parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                return Err(Error::msg("ERR index is not an integer"));
            }
        };
        
        Ok(Move { key, db_index })
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_db_index(&self) -> usize {
        self.db_index
    }

    pub async fn apply(self, handler: &Handler) -> Result<Frame, Error> {
        let key = self.key.clone();
        let db_index = self.db_index;
        
        // 获取当前数据库的发送者
        let current_db_sender = handler.get_session().get_sender();
        
        // 检查键是否存在于当前数据库
        let (exists_tx, exists_rx) = tokio::sync::oneshot::channel();
        let exists_message = crate::store::db::DatabaseMessage::Command { 
            sender: exists_tx, 
            command: crate::command::Command::Exists(crate::cmds::key::exists::Exists { key: key.clone() })
        };
        
        if current_db_sender.send(exists_message).await.is_err() {
            return Ok(Frame::Error("Failed to communicate with current database".to_string()));
        }
        
        let exists_result = match exists_rx.await {
            Ok(frame) => frame,
            Err(_) => return Ok(Frame::Error("Failed to get response from current database".to_string())),
        };
        
        let exists = match exists_result {
            Frame::Integer(1) => true,
            Frame::Integer(0) => false,
            _ => return Ok(Frame::Error("Unexpected response from current database".to_string())),
        };
        
        // 如果键不存在于当前数据库，返回 0
        if !exists {
            return Ok(Frame::Integer(0));
        }
        
        // 检查目标数据库索引是否有效
        if handler.get_args().databases <= db_index {
            return Ok(Frame::Error("ERR DB index is out of range".to_string()));
        }
        
        // 获取目标数据库的发送者
        let target_db_sender = handler.get_db_manager().get_sender(db_index);
        
        // 检查键是否已存在于目标数据库
        let (target_exists_tx, target_exists_rx) = tokio::sync::oneshot::channel();
        let target_exists_message = crate::store::db::DatabaseMessage::Command { 
            sender: target_exists_tx, 
            command: crate::command::Command::Exists(crate::cmds::key::exists::Exists { key: key.clone() })
        };
        
        if target_db_sender.send(target_exists_message).await.is_err() {
            return Ok(Frame::Error("Failed to communicate with target database".to_string()));
        }
        
        let target_exists_result = match target_exists_rx.await {
            Ok(frame) => frame,
            Err(_) => return Ok(Frame::Error("Failed to get response from target database".to_string())),
        };
        
        let target_exists = match target_exists_result {
            Frame::Integer(1) => true,
            Frame::Integer(0) => false,
            _ => return Ok(Frame::Error("Unexpected response from target database".to_string())),
        };
        
        // 如果键已存在于目标数据库，返回 0
        if target_exists {
            return Ok(Frame::Integer(0));
        }
        
        // 从当前数据库获取键值
        let (get_tx, get_rx) = tokio::sync::oneshot::channel();
        let get_message = crate::store::db::DatabaseMessage::Command { 
            sender: get_tx, 
            command: crate::command::Command::Get(crate::cmds::string::get::Get { key: key.clone() })
        };
        
        if current_db_sender.send(get_message).await.is_err() {
            return Ok(Frame::Error("Failed to communicate with current database".to_string()));
        }
        
        let get_result = match get_rx.await {
            Ok(frame) => frame,
            Err(_) => return Ok(Frame::Error("Failed to get value from current database".to_string())),
        };
        
        // 获取键值和类型
        let (value, structure_type) = match get_result {
            Frame::BulkString(value) => {
                // 获取键的类型以确定其结构
                let (type_tx, type_rx) = tokio::sync::oneshot::channel();
                let type_message = crate::store::db::DatabaseMessage::Command { 
                    sender: type_tx, 
                    command: crate::command::Command::Type(crate::cmds::key::r#type::Type { key: key.clone() })
                };
                
                if current_db_sender.send(type_message).await.is_err() {
                    return Ok(Frame::Error("Failed to communicate with current database".to_string()));
                }
                
                let type_result = match type_rx.await {
                    Ok(frame) => frame,
                    Err(_) => return Ok(Frame::Error("Failed to get type from current database".to_string())),
                };
                
                let structure_type = match type_result {
                    Frame::SimpleString(type_str) => type_str,
                    _ => return Ok(Frame::Error("Unexpected type response from current database".to_string())),
                };
                
                (value, structure_type)
            },
            Frame::Null => return Ok(Frame::Integer(0)), // 键不存在
            _ => return Ok(Frame::Error("Unexpected value type".to_string())),
        };
        
        // 在目标数据库设置键值
        let structure = match structure_type.as_str() {
            "string" => crate::store::db::Structure::String(value),
            _ => return Ok(Frame::Error("Unsupported value type for MOVE command".to_string())),
        };
        
        let (set_tx, set_rx) = tokio::sync::oneshot::channel();
        let set_message = crate::store::db::DatabaseMessage::Command { 
            sender: set_tx, 
            command: crate::command::Command::Set(crate::cmds::string::set::Set { 
                key: key.clone(), 
                val: match &structure {
                    crate::store::db::Structure::String(s) => s.clone(),
                    _ => return Ok(Frame::Error("Unsupported value type for MOVE command".to_string())),
                },
                ttl: None,
            })
        };
        
        if target_db_sender.send(set_message).await.is_err() {
            return Ok(Frame::Error("Failed to communicate with target database".to_string()));
        }
        
        let _set_result = match set_rx.await {
            Ok(frame) => frame,
            Err(_) => return Ok(Frame::Error("Failed to set value in target database".to_string())),
        };
        
        // 从当前数据库删除键
        let (del_tx, del_rx) = tokio::sync::oneshot::channel();
        let del_message = crate::store::db::DatabaseMessage::Command { 
            sender: del_tx, 
            command: crate::command::Command::Del(crate::cmds::key::del::Del { keys: vec![key.clone()] })
        };
        
        if current_db_sender.send(del_message).await.is_err() {
            return Ok(Frame::Error("Failed to communicate with current database".to_string()));
        }
        
        let del_result = match del_rx.await {
            Ok(frame) => frame,
            Err(_) => return Ok(Frame::Error("Failed to delete key from current database".to_string())),
        };
        
        match del_result {
            Frame::Integer(1) => Ok(Frame::Integer(1)),
            _ => Ok(Frame::Integer(0)),
        }
    }
}