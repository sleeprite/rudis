use anyhow::Error;
use crate::{frame::Frame, store::db::{Db, Structure}, store::geo::Geo};

pub struct Geoadd {
    key: String,
    items: Vec<(f64, f64, String)>,
}

impl Geoadd {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 5 || (args.len() - 2) % 3 != 0 {
            return Err(Error::msg("ERR wrong number of arguments for 'geoadd' command"));
        }

        let key = args[1].clone();
        let mut items = Vec::new();
        let mut i = 2;

        while i + 2 < args.len() {
            let lon: f64 = args[i].parse().map_err(|_| Error::msg("ERR invalid longitude"))?;
            let lat: f64 = args[i + 1].parse().map_err(|_| Error::msg("ERR invalid latitude"))?;
            let member = args[i + 2].clone();
            items.push((lon, lat, member));
            i += 3;
        }
        Ok(Geoadd { key, items })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        // 使用 entry API 简化逻辑
        let geo = match db.get_mut(&self.key) {
            Some(Structure::Geo(geo)) => geo,
            Some(_) => return Ok(Frame::Error("ERR Operation against a key holding the wrong kind of value".to_string())),
            None => {
                db.insert(self.key.clone(), Structure::Geo(Geo::new()));
                match db.get_mut(&self.key).unwrap() {
                    Structure::Geo(g) => g,
                    _ => unreachable!(),
                }
            }
        };

        let mut added = 0;
        for (lon, lat, member) in self.items {
            match geo.add(member, lon, lat) {
                Ok(n) => added += n,
                Err(e) => return Ok(Frame::Error(e.to_string())),
            }
        }
        Ok(Frame::Integer(added))
    }
}