//! GEOADDJSON - 从 GeoJSON Feature 添加地理点，支持 properties

use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
    store::geo::Geo,
};

/// GEOADDJSON key <geojson_feature_string>
/// GEOADDJSON places '{"type":"Feature","geometry":{"type":"Point","coordinates":[116.3974,39.9093]}}'
pub struct GeoaddJson {
    key: String,
    json_str: String,
}

impl GeoaddJson {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() != 3 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'GEOADDJSON' command",
            ));
        }
        let key = args[1].clone();
        let json_str = args[2].clone();
        Ok(GeoaddJson { key, json_str })
    }

    pub fn apply(self, db: &mut Db) -> Result<Frame, Error> {
        let geo = match db.get_mut(&self.key) {
            Some(Structure::Geo(geo)) => geo,
            Some(_) => {
                return Ok(Frame::Error(
                    "ERR Operation against a key holding the wrong kind of value"
                        .to_string(),
                ))
            }
            None => {
                db.insert(self.key.clone(), Structure::Geo(Geo::new()));
                match db.get_mut(&self.key).unwrap() {
                    Structure::Geo(g) => g,
                    _ => unreachable!(),
                }
            }
        };

        match geo.add_from_geo_json(&self.json_str) {
            Ok(n) => Ok(Frame::Integer(n)),
            Err(e) => Ok(Frame::Error(e.to_string())),
        }
    }
}
