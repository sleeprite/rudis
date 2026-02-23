use anyhow::Error;

use crate::{
    frame::Frame,
    store::db::{Db, Structure},
    store::geo::{GeoRadiusOptions, GeoRadiusResult, GeoUnit},
};

/// GEORADIUS key longitude latitude radius m|km|mi|ft [WITHDIST] [WITHCOORD] [WITHHASH] [COUNT n] [ASC|DESC]
pub struct Georadius {
    key: String,
    longitude: f64,
    latitude: f64,
    radius: f64,
    unit: GeoUnit,
    options: GeoRadiusOptions,
}

impl Georadius {
    pub fn parse_from_frame(frame: Frame) -> Result<Self, Error> {
        let args = frame.get_args();
        if args.len() < 6 {
            return Err(Error::msg(
                "ERR wrong number of arguments for 'georadius' command",
            ));
        }

        let key = args[1].clone();
        let longitude: f64 = args[2].parse().map_err(|_| Error::msg("ERR invalid longitude"))?;
        let latitude: f64 = args[3].parse().map_err(|_| Error::msg("ERR invalid latitude"))?;
        let radius: f64 = args[4].parse().map_err(|_| Error::msg("ERR invalid radius"))?;
        let unit = GeoUnit::from_str(&args[5]);

        let mut options = GeoRadiusOptions::default();
        let mut i = 6;
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

        Ok(Georadius {
            key,
            longitude,
            latitude,
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

        let results = geo.radius(
            self.longitude,
            self.latitude,
            self.radius,
            self.unit,
            &self.options,
        );

        let frames: Vec<Frame> = results
            .into_iter()
            .map(|r| build_result_frame(r, &self.options))
            .collect();

        Ok(Frame::Array(frames))
    }
}

pub(crate) fn build_result_frame(
    r: GeoRadiusResult,
    opt: &GeoRadiusOptions,
) -> Frame {
    if opt.withdist || opt.withcoord || opt.withhash {
        // Redis 顺序: name, distance, hash, coordinates
        let mut arr = vec![Frame::BulkString(r.name)];
        if opt.withdist {
            arr.push(Frame::BulkString(format!("{:.6}", r.distance)));
        }
        if opt.withhash {
            arr.push(Frame::Integer(r.hash as i64));
        }
        if opt.withcoord {
            arr.push(Frame::Array(vec![
                Frame::BulkString(r.longitude.to_string()),
                Frame::BulkString(r.latitude.to_string()),
            ]));
        }
        Frame::Array(arr)
    } else {
        Frame::BulkString(r.name)
    }
}
