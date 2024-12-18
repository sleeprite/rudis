use anyhow::Error;

use crate::{db::Db, frame::Frame};

pub struct Expire {
    key: String,
    ttl: u64
}

impl Expire {

    /// Parses a `Frame` to create an `Expire` command.
    /// 
    /// # Arguments
    /// * `frame` - A `Frame` representing the client's `EXPIRE` command.
    ///
    /// # Returns
    /// A `Result` containing `Self` on success, or an `Error` on failure.
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {

        let args = frame.get_args(); 

        // Check if the frame has at least two arguments: key and ttl
        if args.len() < 3 {
            return Err(Error::msg("ERR wrong number of arguments for 'expire' command"));
        }

        let key = args[1].to_string();

        let ttl = match args[2].parse::<u64>() {
            Ok(val) => val * 1000, // 如果解析成功，将值乘以 1000
            Err(_) => {
                return Err(Error::msg("ERR value is not an integer or out of range"));
            }
        };

        Ok(Expire { 
            key: key, 
            ttl: ttl 
        })
    }

    /// Applies the `Expire` command to the database.
    /// 
    /// # Arguments
    /// * `db` - A mutable reference to the `Db`.
    ///
    /// # Returns
    /// A `Result` containing a `Frame` representing the response on success, or an `Error` on failure.
    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        db.expire(self.key.clone(), self.ttl);
        Ok(Frame::Ok)
    }
}