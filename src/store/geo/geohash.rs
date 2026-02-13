//! Geohash 算法库 (Algorithm Layer)
//! 职责：纯粹的数学计算库，负责 (Lon, Lat) <-> u64 转换。
//! 核心：使用 32 步长生成 64-bit 整数

use std::f64::consts::PI;

// --- 常量定义 ---
const D_R: f64 = PI / 180.0;
const EARTH_RADIUS_IN_METERS: f64 = 6372797.560856;
const MERCATOR_MAX: f64 = 20037726.37;

// Rudis 全精度：32 * 2 = 64 bits
pub const GEO_STEP_MAX: u8 = 32;
// Redis 兼容精度：用于 GEOHASH 命令输出 standard hash string
pub const GEO_STEP_REDIS: u8 = 26;

pub const GEO_LAT_MIN: f64 = -85.05112878;
pub const GEO_LAT_MAX: f64 = 85.05112878;
pub const GEO_LONG_MIN: f64 = -180.0;
pub const GEO_LONG_MAX: f64 = 180.0;

// --- 基础结构 ---

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GeoHashBits {
    pub bits: u64,
    pub step: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GeoHashRange {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GeoHashNeighbors {
    pub north: GeoHashBits,
    pub east: GeoHashBits,
    pub west: GeoHashBits,
    pub south: GeoHashBits,
    pub north_east: GeoHashBits,
    pub south_east: GeoHashBits,
    pub north_west: GeoHashBits,
    pub south_west: GeoHashBits,
}

// --- 核心位操作 ---

/// Interleave: 将 x(Lat) 和 y(Long) 的低 32 位交错生成 64 位整数
fn interleave64(xlo: u32, ylo: u32) -> u64 {
    static B: [u64; 5] = [0x5555555555555555, 0x3333333333333333, 0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF, 0x0000FFFF0000FFFF];
    static S: [u32; 5] = [1, 2, 4, 8, 16];

    let mut x = xlo as u64; // Lat
    let mut y = ylo as u64; // Long

    x = (x | (x << S[4])) & B[4]; y = (y | (y << S[4])) & B[4];
    x = (x | (x << S[3])) & B[3]; y = (y | (y << S[3])) & B[3];
    x = (x | (x << S[2])) & B[2]; y = (y | (y << S[2])) & B[2];
    x = (x | (x << S[1])) & B[1]; y = (y | (y << S[1])) & B[1];
    x = (x | (x << S[0])) & B[0]; y = (y | (y << S[0])) & B[0];

    x | (y << 1)
}

#[allow(dead_code)]
fn deinterleave64(interleaved: u64) -> u64 {
    static B: [u64; 6] = [0x5555555555555555, 0x3333333333333333, 0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF, 0x0000FFFF0000FFFF, 0x00000000FFFFFFFF];
    static S: [u32; 6] = [0, 1, 2, 4, 8, 16];

    let mut x = interleaved;
    let mut y = interleaved >> 1;

    x = (x | (x >> S[0])) & B[0]; y = (y | (y >> S[0])) & B[0];
    x = (x | (x >> S[1])) & B[1]; y = (y | (y >> S[1])) & B[1];
    x = (x | (x >> S[2])) & B[2]; y = (y | (y >> S[2])) & B[2];
    x = (x | (x >> S[3])) & B[3]; y = (y | (y >> S[3])) & B[3];
    x = (x | (x >> S[4])) & B[4]; y = (y | (y >> S[4])) & B[4];
    x = (x | (x >> S[5])) & B[5]; y = (y | (y >> S[5])) & B[5];

    x | (y << 32)
}

/// 编码与解码
pub fn geohash_encode_wgs84(longitude: f64, latitude: f64, step: u8) -> Option<GeoHashBits> {
    if step > 32 || step == 0 { return None; }
    if longitude > GEO_LONG_MAX || longitude < GEO_LONG_MIN || latitude > GEO_LAT_MAX || latitude < GEO_LAT_MIN {
        return None;
    }

    let lat_offset = (latitude - GEO_LAT_MIN) / (GEO_LAT_MAX - GEO_LAT_MIN);
    let long_offset = (longitude - GEO_LONG_MIN) / (GEO_LONG_MAX - GEO_LONG_MIN);

    let step_pow = (1u64 << step) as f64;

    Some(GeoHashBits {
        bits: interleave64(
            (lat_offset * step_pow) as u32,
            (long_offset * step_pow) as u32
        ),
        step
    })
}

/// 邻居计算
fn geohash_move_x(hash: &mut GeoHashBits, d: i8) {
    if d == 0 { return; }
    let mut x = hash.bits & 0xaaaaaaaaaaaaaaaa;
    let y = hash.bits & 0x5555555555555555;
    let zz = 0x5555555555555555 >> (64 - hash.step * 2);
    if d > 0 { x = x + (zz + 1); } else { x = x | zz; x = x - (zz + 1); }
    x &= 0xaaaaaaaaaaaaaaaa >> (64 - hash.step * 2);
    hash.bits = x | y;
}

fn geohash_move_y(hash: &mut GeoHashBits, d: i8) {
    if d == 0 { return; }
    let x = hash.bits & 0xaaaaaaaaaaaaaaaa;
    let mut y = hash.bits & 0x5555555555555555;
    let zz = 0xaaaaaaaaaaaaaaaa >> (64 - hash.step * 2);
    if d > 0 { y = y + (zz + 1); } else { y = y | zz; y = y - (zz + 1); }
    y &= 0x5555555555555555 >> (64 - hash.step * 2);
    hash.bits = x | y;
}

pub fn geohash_neighbors(hash: &GeoHashBits, neighbors: &mut GeoHashNeighbors) {
    neighbors.east = *hash; neighbors.west = *hash;
    neighbors.north = *hash; neighbors.south = *hash;
    neighbors.south_east = *hash; neighbors.south_west = *hash;
    neighbors.north_east = *hash; neighbors.north_west = *hash;

    geohash_move_x(&mut neighbors.east, 1); geohash_move_y(&mut neighbors.east, 0);
    geohash_move_x(&mut neighbors.west, -1); geohash_move_y(&mut neighbors.west, 0);
    geohash_move_x(&mut neighbors.south, 0); geohash_move_y(&mut neighbors.south, -1);
    geohash_move_x(&mut neighbors.north, 0); geohash_move_y(&mut neighbors.north, 1);

    geohash_move_x(&mut neighbors.north_west, -1); geohash_move_y(&mut neighbors.north_west, 1);
    geohash_move_x(&mut neighbors.north_east, 1); geohash_move_y(&mut neighbors.north_east, 1);
    geohash_move_x(&mut neighbors.south_east, 1); geohash_move_y(&mut neighbors.south_east, -1);
    geohash_move_x(&mut neighbors.south_west, -1); geohash_move_y(&mut neighbors.south_west, -1);
}

// --- 关键辅助函数：BTree Range 计算 ---

/// 将 HashBits 转换为 u64 键 (左对齐)
pub fn geohash_bits_to_u64(hash: GeoHashBits) -> u64 {
    let shift = 64u32.saturating_sub((hash.step as u32) * 2);
    hash.bits << shift
}

/// 计算 HashBits 覆盖的 u64 范围 [min, max]
pub fn geohash_to_u64_range(hash: GeoHashBits) -> (u64, u64) {
    let min = geohash_bits_to_u64(hash);
    let shift = 64u32.saturating_sub((hash.step as u32) * 2);
    let mask = if shift == 64 { u64::MAX } else { (1u64 << shift) - 1 };
    (min, min | mask)
}

/// 工具函数：距离与格式转换
pub fn geohash_estimate_steps_by_radius(mut range_meters: f64, lat: f64) -> u8 {
    if range_meters == 0.0 { return 32; }
    let mut step = 1;
    while range_meters < MERCATOR_MAX {
        range_meters *= 2.0;
        step += 1;
    }
    step -= 2;
    if lat > 66.0 || lat < -66.0 { step -= 1; if lat > 80.0 || lat < -80.0 { step -= 1; } }
    if step < 1 { 1 } else if step > 32 { 32 } else { step as u8 }
}

#[inline]
fn deg_rad(ang: f64) -> f64 { ang * D_R }

pub fn geohash_get_distance(lon1d: f64, lat1d: f64, lon2d: f64, lat2d: f64) -> f64 {
    let lon1r = deg_rad(lon1d);
    let lon2r = deg_rad(lon2d);
    let v = ((lon2r - lon1r) / 2.0).sin();
    if v == 0.0 { return EARTH_RADIUS_IN_METERS * (deg_rad(lat2d) - deg_rad(lat1d)).abs(); }
    let lat1r = deg_rad(lat1d);
    let lat2r = deg_rad(lat2d);
    let u = ((lat2r - lat1r) / 2.0).sin();
    let a = u * u + lat1r.cos() * lat2r.cos() * v * v;
    2.0 * EARTH_RADIUS_IN_METERS * a.sqrt().asin()
}

const BASE32_CHARS: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";

pub fn geohash_bits_to_base32(hash: GeoHashBits) -> String {
    let bits = hash.bits;
    let step = hash.step as usize;
    let num_bits = step * 2;
    let mut result = String::with_capacity(11);
    let mut remaining = bits << (64 - num_bits);
    for _ in 0..((num_bits + 4) / 5) {
        let idx = (remaining >> 59) as usize;
        result.push(BASE32_CHARS[idx] as char);
        remaining <<= 5;
    }
    result
}

/// 仅用于兼容 Redis 协议输出
pub fn geohash_u64_to_redis_52bit(hash_u64: u64) -> u64 {
    hash_u64 >> 12 // 64 - 52 = 12
}