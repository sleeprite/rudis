use anyhow::Error;
use std::collections::HashMap;
use crate::{frame::Frame, store::db::{Db, Structure, Vector}};

#[derive(Debug)]
pub struct Vadd {
    key: String,
    id: String,
    vector: Vec<f32>,
    ttl: Option<u64>,
}

impl Vadd {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        
        let args = frame.get_args();
       
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'VADD' command"));
        }
       
        let key = args[1].to_string();
        let id = args[2].to_string();

        // 获取 Vec 参数
        let mut idx = 3;
        let mut vector = Vec::new();
        while idx < args.len() {
            match args[idx].parse::<f32>() {
                Ok(val) => vector.push(val),
                Err(_) => {
                    if args[idx].to_uppercase() == "EX" || args[idx].to_uppercase() == "PX" {
                        break;
                    } else {
                        return Err(Error::msg("Invalid vector value"));
                    }
                }
            }
            idx += 1;
        }

        if vector.is_empty() {
            return Err(Error::msg("ERR vector cannot be empty"));
        }

        // 获取 Ttl 参数
        let mut ttl = None;
        while idx < args.len() {
            let option = args[idx].to_uppercase();
            if option == "EX" || option == "PX" {
                if idx + 1 >= args.len() {
                    return Err(Error::msg("ERR syntax error"));
                }
                let value = args[idx + 1].parse::<u64>()?;
                ttl = Some(match option.as_str() {
                    "EX" => value * 1000,
                    "PX" => value,
                    _ => unreachable!(),
                });
                idx += 2;
            } else {
                return Err(Error::msg("ERR syntax error"));
            }
        }

        if idx != args.len() {
            return Err(Error::msg("ERR syntax error"));
        }

        Ok(Self { key, id, vector, ttl })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {

        let key_clone = self.key.clone();
        let structure = db.records.get_mut(&key_clone);

        match structure {
            Some(Structure::VectorCollection(v)) => {
            
                if self.vector.len() != v.dimension {
                    return Err(Error::msg("ERR vector dimension mismatch"));
                }
                
                let n = self.vector.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
                v.vectors.insert(self.id.clone(), self.vector);
                v.norms.insert(self.id.clone(), n);
            }
            Some(_) => return Err(Error::msg("WRONGTYPE Operation against a key holding the wrong kind of value")),
            None => {
      
                let dimension = self.vector.len();
                let norm = self.vector.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
                
                let mut vectors: HashMap<_, _> = HashMap::new();
                vectors.insert(self.id.clone(), self.vector);
                
                let mut norms = HashMap::new();
                norms.insert(self.id.clone(), norm);
                
                db.records.insert(
                    self.key.clone(),
                    Structure::VectorCollection(Vector {
                        dimension,
                        vectors,
                        norms,
                    }),
                );
            }
        }

        // 设置 TTL
        if let Some(ttl) = self.ttl {
            db.expire(self.key, ttl);
        }

        Ok(Frame::SimpleString("OK".into()))
    }
}