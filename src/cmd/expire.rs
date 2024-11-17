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

        let args = frame.get_args(); // Assuming Frame has a method to get arguments as a slice

        // Check if the frame has at least two arguments: key and ttl
        if args.len() < 2 {
            return Err(Error::msg("Insufficient arguments for EXPIRE command"));
        }

        let key = args[1].to_string();
        let ttl = match args[2].parse::<u64>() {
            Ok(val) => val * 1000, // 如果解析成功，将值乘以 1000
            Err(_) => {
                // 如果解析失败，返回错误
                return Err(anyhow::Error::msg("Invalid TTL value"));
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