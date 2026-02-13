use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
    store::geo::{GeoRadiusOptions, GeoUnit},
};

/// GEORADIUSBYMEMBER key member radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT n] [ASC|DESC]
pub struct Georadiusbymember {
    key: String,
    member: String,
    radius: f64,
    unit: GeoUnit,
    options: GeoRadiusOptions,
}

impl Georadiusbymember {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 5 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'georadiusbymember' command",
            ));
        }

        let key = args[1].clone();
        let member = args[2].clone();
        let radius: f64 = args[3].parse().map_err(|_| Error::msg("ERR invalid radius"))?;
        let unit = GeoUnit::from_str(&args[4]);

        let mut options = GeoRadiusOptions::default();
        let mut i = 5;
        while i < args.len() {
            let arg = args[i].to_uppercase();
            match arg.as_str() {
                "WITHDIST" => options.withdist = true,
                "WITHCOORD" => options.withcoord = true,
                "WITHHASH" => options.withhash = true,
                "ASC" => options.sort_asc = true,
                "DESC" => options.sort_desc = true,
                "COUNT" => {
                    i += 1;
                    if i < args.len() {
                        options.count = args[i].parse().ok();
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Ok(Georadiusbymember {
            key,
            member,
            radius,
            unit,
            options,
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
            None => return Ok(Frame::Array(vec![])),
        };

        let results = match geo.radius_by_member(&self.member, self.radius, self.unit, &self.options) {
            Some(r) => r,
            None => return Ok(Frame::Array(vec![])),
        };

        let frames: Vec<Frame> = results
            .into_iter()
            .map(|r| super::georadius::build_result_frame(r, &self.options))
            .collect();

        Ok(Frame::Array(frames))
    }
}
