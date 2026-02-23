use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
    store::geo::GeoUnit,
};

/// GEODIST key member1 member2 [m|km|mi|ft]
pub struct Geodist {
    key: String,
    member1: String,
    member2: String,
    unit: GeoUnit,
}

impl Geodist {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 4 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'geodist' command",
            ));
        }
        let key = args[1].clone();
        let member1 = args[2].clone();
        let member2 = args[3].clone();
        let unit = args
            .get(4)
            .map(|s| GeoUnit::from_str(s))
            .unwrap_or(GeoUnit::Meters);

        Ok(Geodist {
            key,
            member1,
            member2,
            unit,
        })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let geo = match db.get_mut(&self.key) {
            Some(Structure::Geo(geo)) => geo,
            Some(_) => {
                return Ok(Frame::Error(
                    "ERR Operation against a key holding the wrong kind of value".to_string(),
                ));
            }
            None => return Ok(Frame::Null),
        };

        match geo.dist(&self.member1, &self.member2, self.unit) {
            Some(d) => Ok(Frame::BulkString(format!("{:.6}", d))),
            None => Ok(Frame::Null),
        }
    }
}
