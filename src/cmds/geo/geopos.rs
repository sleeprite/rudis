use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
};

/// GEOPOS key member [member ...]
pub struct Geopos {
    key: String,
    members: Vec<String>,
}

impl Geopos {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 3 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'geopos' command",
            ));
        }
        let key = args[1].clone();
        let members = args[2..].to_vec();
        Ok(Geopos { key, members })
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
                // 键不存在，返回全 nil 数组
                let results: Vec<Frame> = self.members.iter().map(|_| Frame::Null).collect();
                return Ok(Frame::Array(results));
            }
        };

        let results: Vec<Frame> = self
            .members
            .iter()
            .map(|m| {
                geo.pos(m).map_or(Frame::Null, |(lon, lat)| {
                    Frame::Array(vec![
                        Frame::BulkString(lon.to_string()),
                        Frame::BulkString(lat.to_string()),
                    ])
                })
            })
            .collect();

        Ok(Frame::Array(results))
    }
}
