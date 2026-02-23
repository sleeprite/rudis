use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::Included;
use anyhow::{Error, Result};
use bincode::{BorrowDecode, Decode, Encode};
use geojson::{GeoJson, Geometry, Value as GeoJsonValue};

use super::geohash::{
    geohash_encode_wgs84, geohash_neighbors, geohash_estimate_steps_by_radius,
    geohash_to_u64_range, geohash_get_distance, geohash_bits_to_base32,
    geohash_bits_to_u64, geohash_u64_to_redis_52bit, 
    GeoHashNeighbors,
    GEO_STEP_MAX, GEO_STEP_REDIS,
};
use super::types::GeoPoint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeoUnit { Meters, Kilometers, Miles, Feet }

impl GeoUnit {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "km" | "kilometers" => GeoUnit::Kilometers,
            "mi" | "miles" => GeoUnit::Miles,
            "ft" | "feet" => GeoUnit::Feet,
            _ => GeoUnit::Meters,
        }
    }
    fn to_meters(&self, val: f64) -> f64 {
        match self {
            GeoUnit::Meters => val,
            GeoUnit::Kilometers => val * 1000.0,
            GeoUnit::Miles => val * 1609.34,
            GeoUnit::Feet => val * 0.3048,
        }
    }
    fn from_meters(&self, meters: f64) -> f64 {
        match self {
            GeoUnit::Meters => meters,
            GeoUnit::Kilometers => meters / 1000.0,
            GeoUnit::Miles => meters / 1609.34,
            GeoUnit::Feet => meters / 0.3048,
        }
    }
}

#[derive(Debug, Default)]
pub struct GeoRadiusOptions {
    pub withdist: bool,
    pub withcoord: bool,
    pub withhash: bool,
    pub count: Option<usize>,
    pub sort_asc: bool,
    pub sort_desc: bool,
}

#[derive(Debug, Clone)]
pub struct GeoRadiusResult {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub hash: u64,
}

/// Geo 核心结构
#[derive(Debug, Default, Clone)]
pub struct Geo {
    spatial_index: BTreeMap<u64, Vec<GeoPoint>>,
    member_index: HashMap<String, u64>,
}

impl Geo {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add(&mut self, name: String, longitude: f64, latitude: f64) -> Result<i64> {
        let is_new = !self.member_index.contains_key(&name);
        let point = GeoPoint::new(name, longitude, latitude);
        self.insert_point(point).map_err(Error::msg)?;
        Ok(if is_new { 1 } else { 0 })
    }

    pub fn add_from_geo_json(&mut self, json_str: &str) -> Result<i64> {
        let geo_json: GeoJson = json_str.parse().map_err(|e: geojson::Error| Error::msg(e.to_string()))?;

        if let GeoJson::Feature(feature) = geo_json {
            // 彻底解构 feature，避免 partial move 错误
            // 把 geometry 和 properties 的所有权拿出来，不再依赖 feature 变量
            let geometry = feature.geometry;
            let properties = feature.properties;

            let p = match geometry {
                Some(
                    Geometry {
                        value: GeoJsonValue::Point(vec), .. 
                    }) => vec,
                _ => return Err(Error::msg("Only Point geometry supported")),
            };

            // 手动从 properties Option Map 中查找 name，如果找不到用 "unknown"
            let name = properties.as_ref()
                .and_then(|props| props.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            let point = GeoPoint {
                longitude: p[0],
                latitude: p[1],
                name,
                properties, // 所有权直接转移给 GeoPoint
            };

            let is_new = !self.member_index.contains_key(&point.name);
            self.insert_point(point).map_err(Error::msg)?;
            Ok(if is_new { 1 } else { 0 })
        } else {
            Err(Error::msg("Input must be a GeoJSON Feature"))
        }
    }

    fn insert_point(&mut self, point: GeoPoint) -> Result<(), String> {
        let hash_bits = geohash_encode_wgs84(point.longitude, point.latitude, GEO_STEP_MAX)
            .ok_or("Coordinates out of range")?;
        let hash_u64 = geohash_bits_to_u64(hash_bits);
        let name = point.name.clone();

        if let Some(old_hash) = self.member_index.get(&name) {
            self.remove_from_spatial(*old_hash, &name);
        }

        self.member_index.insert(name, hash_u64);
        self.spatial_index
            .entry(hash_u64)
            .or_insert_with(Vec::new)
            .push(point);
        Ok(())
    }

    pub fn rem(&mut self, name: &str) -> i64 {
        if let Some(hash) = self.member_index.remove(name) {
            self.remove_from_spatial(hash, name);
            1
        } else {
            0
        }
    }

    pub fn len(&self) -> usize {
        self.member_index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.member_index.is_empty()
    }

    fn remove_from_spatial(&mut self, hash: u64, name: &str) {
        if let Some(bucket) = self.spatial_index.get_mut(&hash) {
            bucket.retain(|p| p.name != name);
            if bucket.is_empty() {
                self.spatial_index.remove(&hash);
            }
        }
    }

    pub fn pos(&self, name: &str) -> Option<(f64, f64)> {
        let p = self.get_position(name)?;
        Some((p.longitude, p.latitude))
    }

    fn get_position(&self, name: &str) -> Option<&GeoPoint> {
        let hash = self.member_index.get(name)?;
        let bucket = self.spatial_index.get(hash)?;
        bucket.iter().find(|p| p.name == name)
    }

    pub fn dist(&self, member1: &str, member2: &str, unit: GeoUnit) -> Option<f64> {
        let p1 = self.get_position(member1)?;
        let p2 = self.get_position(member2)?;
        let meters = geohash_get_distance(p1.longitude, p1.latitude, p2.longitude, p2.latitude);
        Some(unit.from_meters(meters))
    }

    pub fn hash(&self, member: &str) -> Option<String> {
        let p = self.get_position(member)?;
        let hash = geohash_encode_wgs84(p.longitude, p.latitude, GEO_STEP_REDIS)?;
        Some(geohash_bits_to_base32(hash))
    }

    pub fn radius_by_member(
        &self,
        member: &str,
        radius: f64,
        unit: GeoUnit,
        options: &GeoRadiusOptions,
    ) -> Option<Vec<GeoRadiusResult>> {
        let (lon, lat) = self.pos(member)?;
        Some(self.radius(lon, lat, radius, unit, options))
    }

    pub fn radius(
        &self,
        longitude: f64,
        latitude: f64,
        radius: f64,
        unit: GeoUnit,
        options: &GeoRadiusOptions,
    ) -> Vec<GeoRadiusResult> {
        let radius_m = unit.to_meters(radius);
        let points = self.search_spatial_index(longitude, latitude, radius_m);

        let mut results: Vec<GeoRadiusResult> = points.into_iter().map(|p| {
            let dist_m = geohash_get_distance(longitude, latitude, p.longitude, p.latitude);

            let hash_bits = geohash_encode_wgs84(p.longitude, p.latitude, GEO_STEP_MAX);
            let hash_u64 = hash_bits.map(geohash_bits_to_u64).unwrap_or(0);

            GeoRadiusResult {
                name: p.name.clone(),
                longitude: p.longitude,
                latitude: p.latitude,
                distance: unit.from_meters(dist_m),
                hash: geohash_u64_to_redis_52bit(hash_u64),
            }
        }).collect();

        if options.sort_asc {
            results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        } else if options.sort_desc {
            results.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
        }
        if let Some(n) = options.count {
            results.truncate(n);
        }
        results
    }

    fn search_spatial_index(&self, lon: f64, lat: f64, radius_m: f64) -> Vec<&GeoPoint> {
        let mut results = Vec::new();
        let step = geohash_estimate_steps_by_radius(radius_m, lat);

        let center_hash = match geohash_encode_wgs84(lon, lat, step) {
            Some(h) => h,
            None => return results,
        };

        let mut neighbors = GeoHashNeighbors::default();
        geohash_neighbors(&center_hash, &mut neighbors);

        let areas = [
            center_hash,
            neighbors.north, neighbors.south, neighbors.east, neighbors.west,
            neighbors.north_east, neighbors.north_west, neighbors.south_east, neighbors.south_west,
        ];

        for area_hash in areas.iter() {
            let (min_u64, max_u64) = geohash_to_u64_range(*area_hash);
            for (_, bucket) in self.spatial_index.range((Included(&min_u64), Included(&max_u64))) {
                for point in bucket {
                    let dist = geohash_get_distance(lon, lat, point.longitude, point.latitude);
                    if dist <= radius_m {
                        results.push(point);
                    }
                }
            }
        }
        results
    }
}

/// 持久化 (Encode/Decode for Geo)
impl Encode for Geo {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        let items: Vec<(u64, Vec<GeoPoint>)> = self.spatial_index.iter().map(|(k, v)| (*k, v.clone())).collect();
        let members: Vec<(String, u64)> = self.member_index.iter().map(|(k, v)| (k.clone(), *v)).collect();
        items.encode(encoder)?;
        members.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Geo {
    fn decode<D: bincode::de::Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let items: Vec<(u64, Vec<GeoPoint>)> = Vec::decode(decoder)?;
        let members: Vec<(String, u64)> = Vec::decode(decoder)?;
        Ok(Self {
            spatial_index: items.into_iter().collect(),
            member_index: members.into_iter().collect(),
        })
    }
}
impl<'de, Context> BorrowDecode<'de, Context> for Geo {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let items: Vec<(u64, Vec<GeoPoint>)> = Vec::borrow_decode(decoder)?;
        let members: Vec<(String, u64)> = Vec::borrow_decode(decoder)?;
        Ok(Self {
            spatial_index: items.into_iter().collect(),
            member_index: members.into_iter().collect(),
        })
    }
}