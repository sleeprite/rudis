use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
};

/// GEOHASH key member [member ...]
pub struct Geohash {
    key: String,
    members: Vec<String>,
}

impl Geohash {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 3 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'geohash' command",
            ));
        }
        let key = args[1].clone();
        let members = args[2..].to_vec();
        Ok(Geohash { key, members })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let geo = match db.get_mut(&self.key) {
            Some(Structure::Geo(geo)) => geo,
            Some(_) => {
                return Ok(Frame::Error(
                    "ERR Operation against a key holding the wrong kind of value".to_string(),
                ));
            }
            None => {
                let results: Vec<Frame> = self.members.iter().map(|_| Frame::Null).collect();
                return Ok(Frame::Array(results));
            }
        };

        let results: Vec<Frame> = self
            .members
            .iter()
            .map(|m| geo.hash(m).map_or(Frame::Null, Frame::BulkString))
            .collect();

        Ok(Frame::Array(results))
    }
}
