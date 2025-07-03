use anyhow::Error;
use crate::{store::db::{Db, Structure}, frame::Frame};

pub struct Mset {
    key_vals: Vec<(String, String)>,
}

impl Mset {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args_from_index(1);

        if args.len() % 2 != 0 {
            return Err(Error::msg("ERR wrong number of arguments for 'mset' command"));
        }

        let mut key_vals = Vec::new();
        
        for i in (0..args.len()).step_by(2) {
            let key = args[i].to_string();
            let val = args[i + 1].to_string();
            key_vals.push((key, val));
        }

        Ok(Mset { key_vals })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        for (key, val) in self.key_vals {
            db.insert(key, Structure::String(val));
        }
        Ok(Frame::Ok)
    }
}