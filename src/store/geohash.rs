use std::f64;
use std::f64::consts::PI;

// --- 常量定义(参考Redis 源码 src/geohash_helper.h) ---
const D_R: f64 = PI / 180.0;
const EARTH_RADIUS_IN_METERS: f64 = 6372797.560856; // WGS84 地球半径
const MERCATOR_MAX: f64 = 20037726.37; // 墨卡托投影最大值

// --- 常量定义 (参考Redis 源码 src/geohash.h) ---

// Redis 固定使用 26 步长 (26 * 2 = 52 bits)，正好能塞进 double 的尾数部分
pub const GEO_STEP_MAX: u8 = 26;

// WGS84 坐标系极限值 (Limits from EPSG:900913 / EPSG:3785 / OSGEO:41001)
pub const GEO_LAT_MIN: f64 = -85.05112878;
pub const GEO_LAT_MAX: f64 = 85.05112878;
pub const GEO_LONG_MIN: f64 = -180.0;
pub const GEO_LONG_MAX: f64 = 180.0;

// 方向枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeoDirection {
    North = 0,
    East,
    West,
    South,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast,
}

// 核心结构：存储 Geohash 的二进制位和步长
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GeoHashBits {
    pub bits: u64,
    pub step: u8,
}

impl GeoHashBits {
    // 判断是否为空
    pub fn is_zero(&self) -> bool {
        self.bits == 0 && self.step == 0
    }

    // 重置为 0
    pub fn zero(&mut self) {
        self.bits = 0;
        self.step = 0;
    }
}

// 经纬度范围结构体
#[derive(Debug, Clone, Copy, Default)]
pub struct GeoHashRange {
    pub min: f64,
    pub max: f64,
}

impl GeoHashRange {
    pub fn is_zero(&self) -> bool {
        self.min == 0.0 && self.max == 0.0
    }
}

// 解码后的区域结构体
#[derive(Debug, Clone, Copy, Default)]
pub struct GeoHashArea {
    pub hash: GeoHashBits,
    pub longitude: GeoHashRange,
    pub latitude: GeoHashRange,
}

// 邻居节点结构体（九宫格）
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

// --- 核心算法实现 (参考Redis 源码 src/geohash.c) ---

/// 位交错函数 (Interleave)
/// 将 x 和 y 的低位比特交错排列：x 在偶数位，y 在奇数位。
/// 经典的 "Magic Number" 算法，效率极高。
/// 来源: https://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN
fn interleave64(xlo: u32, ylo: u32) -> u64 {
    // 魔法数字表
    static B: [u64; 5] = [
        0x5555555555555555, 0x3333333333333333,
        0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF,
        0x0000FFFF0000FFFF
    ];
    static S: [u32; 5] = [1, 2, 4, 8, 16];

    let mut x = xlo as u64;
    let mut y = ylo as u64;

    // 通过一系列位移和掩码操作，把比特位拉开
    x = (x | (x << S[4])) & B[4];
    y = (y | (y << S[4])) & B[4];

    x = (x | (x << S[3])) & B[3];
    y = (y | (y << S[3])) & B[3];

    x = (x | (x << S[2])) & B[2];
    y = (y | (y << S[2])) & B[2];

    x = (x | (x << S[1])) & B[1];
    y = (y | (y << S[1])) & B[1];

    x = (x | (x << S[0])) & B[0];
    y = (y | (y << S[0])) & B[0];

    // 最后合并：x 在偶数位，y 左移一位占据奇数位
    x | (y << 1)
}

/// 位去交错函数 (De-interleave)
/// interleave64 的逆运算，用于将一个 hash 值还原为经度和纬度的整数部分。
fn deinterleave64(interleaved: u64) -> u64 {
    static B: [u64; 6] = [
        0x5555555555555555, 0x3333333333333333,
        0x0F0F0F0F0F0F0F0F, 0x00FF00FF00FF00FF,
        0x0000FFFF0000FFFF, 0x00000000FFFFFFFF
    ];
    static S: [u32; 6] = [0, 1, 2, 4, 8, 16];

    let mut x = interleaved;
    let mut y = interleaved >> 1;

    x = (x | (x >> S[0])) & B[0];
    y = (y | (y >> S[0])) & B[0];

    x = (x | (x >> S[1])) & B[1];
    y = (y | (y >> S[1])) & B[1];

    x = (x | (x >> S[2])) & B[2];
    y = (y | (y >> S[2])) & B[2];

    x = (x | (x >> S[3])) & B[3];
    y = (y | (y >> S[3])) & B[3];

    x = (x | (x >> S[4])) & B[4];
    y = (y | (y >> S[4])) & B[4];

    x = (x | (x >> S[5])) & B[5];
    y = (y | (y >> S[5])) & B[5];

    x | (y << 32)
}

/// 初始化经纬度范围为地球极限值
pub fn geohash_get_coord_range() -> (GeoHashRange, GeoHashRange) {
    let long_range = GeoHashRange { min: GEO_LONG_MIN, max: GEO_LONG_MAX };
    let lat_range = GeoHashRange { min: GEO_LAT_MIN, max: GEO_LAT_MAX };
    (long_range, lat_range)
}

/// 核心编码函数：经纬度 -> GeoHashBits
pub fn geohash_encode(
    long_range: &GeoHashRange,
    lat_range: &GeoHashRange,
    longitude: f64,
    latitude: f64,
    step: u8,
) -> Option<GeoHashBits> {
    // 参数检查
    if step > 32 || step == 0 || lat_range.is_zero() || long_range.is_zero() {
        return None;
    }

    // 检查经纬度是否超出地球范围
    if longitude > GEO_LONG_MAX || longitude < GEO_LONG_MIN ||
        latitude > GEO_LAT_MAX || latitude < GEO_LAT_MIN
    {
        return None;
    }

    // 检查是否在指定范围内
    if latitude < lat_range.min || latitude > lat_range.max ||
        longitude < long_range.min || longitude > long_range.max
    {
        return None;
    }

    // 计算归一化偏移量
    let mut lat_offset = (latitude - lat_range.min) / (lat_range.max - lat_range.min);
    let mut long_offset = (longitude - long_range.min) / (long_range.max - long_range.min);

    let step_pow = (1u64 << step) as f64;
    lat_offset *= step_pow;
    long_offset *= step_pow;

    Some(GeoHashBits {
        bits: interleave64(lat_offset as u32, long_offset as u32),
        step
    })
}



/// 便捷函数：使用 WGS84 坐标系进行编码
pub fn geohash_encode_wgs84(longitude: f64, latitude: f64, step: u8) -> Option<GeoHashBits> {
    let (long_range, lat_range) = geohash_get_coord_range();
    geohash_encode(&long_range, &lat_range, longitude, latitude, step)
}

/// 核心解码函数：GeoHashBits -> 经纬度范围
pub fn geohash_decode(
    long_range: GeoHashRange,
    lat_range: GeoHashRange,
    hash: GeoHashBits,
) -> Option<GeoHashArea> {
    if hash.is_zero() || lat_range.is_zero() || long_range.is_zero() {
        return None;
    }

    let step = hash.step;
    let hash_sep = deinterleave64(hash.bits);

    let lat_scale = lat_range.max - lat_range.min;
    let long_scale = long_range.max - long_range.min;

    let ilato = hash_sep as u32;
    let ilono = (hash_sep >> 32) as u32;

    let step_pow = (1u64 << step) as f64;

    let mut area = GeoHashArea::default();
    area.hash = hash;
    area.latitude.min = lat_range.min + (ilato as f64 * 1.0 / step_pow) * lat_scale;
    area.latitude.max = lat_range.min + ((ilato + 1) as f64 * 1.0 / step_pow) * lat_scale;
    area.longitude.min = long_range.min + (ilono as f64 * 1.0 / step_pow) * long_scale;
    area.longitude.max = long_range.min + ((ilono + 1) as f64 * 1.0 / step_pow) * long_scale;

    Some(area)
}

/// 辅助函数：将解码后的区域转换为中心点坐标
pub fn geohash_decode_area_to_long_lat(area: &GeoHashArea) -> Option<[f64; 2]> {
    let mut xy = [0.0; 2];
    xy[0] = (area.longitude.min + area.longitude.max) / 2.0;
    if xy[0] > GEO_LONG_MAX { xy[0] = GEO_LONG_MAX; }
    if xy[0] < GEO_LONG_MIN { xy[0] = GEO_LONG_MIN; }

    xy[1] = (area.latitude.min + area.latitude.max) / 2.0;
    if xy[1] > GEO_LAT_MAX { xy[1] = GEO_LAT_MAX; }
    if xy[1] < GEO_LAT_MIN { xy[1] = GEO_LAT_MIN; }

    Some(xy)
}

/// 直接解码为 WGS84 坐标
pub fn geohash_decode_to_long_lat_wgs84(hash: GeoHashBits) -> Option<[f64; 2]> {
    let (long_range, lat_range) = geohash_get_coord_range();
    // 使用 ? 操作符进行链式调用，非常 Rustacean！
    let area = geohash_decode(long_range, lat_range, hash)?;
    geohash_decode_area_to_long_lat(&area)
}

// --- 邻居计算算法 ---

/// 在 X 轴（经度方向）移动 Hash
/// 这是一个纯位运算，不涉及浮点数，性能极高
fn geohash_move_x(hash: &mut GeoHashBits, d: i8) {
    if d == 0 { return; }

    let mut x = hash.bits & 0xaaaaaaaaaaaaaaaa; // 提取偶数位 (经度位)
    let y = hash.bits & 0x5555555555555555;     // 提取奇数位 (纬度位)

    // zz 是掩码，用于处理进位溢出
    let zz = 0x5555555555555555 >> (64 - hash.step * 2);

    if d > 0 {
        x = x + (zz + 1); // 向东移
    } else {
        x = x | zz;
        x = x - (zz + 1); // 向西移
    }

    // 重新组合
    x &= 0xaaaaaaaaaaaaaaaa >> (64 - hash.step * 2);
    hash.bits = x | y;
}

/// 在 Y 轴（纬度方向）移动 Hash
fn geohash_move_y(hash: &mut GeoHashBits, d: i8) {
    if d == 0 { return; }

    let x = hash.bits & 0xaaaaaaaaaaaaaaaa;
    let mut y = hash.bits & 0x5555555555555555;

    let zz = 0xaaaaaaaaaaaaaaaa >> (64 - hash.step * 2);

    if d > 0 {
        y = y + (zz + 1); // 向北移
    } else {
        y = y | zz;
        y = y - (zz + 1); // 向南移
    }

    y &= 0x5555555555555555 >> (64 - hash.step * 2);
    hash.bits = x | y;
}

/// 计算当前 Hash 周围的 8 个邻居 (九宫格)
pub fn geohash_neighbors(hash: &GeoHashBits, neighbors: &mut GeoHashNeighbors) {
    // 先把所有邻居初始化为当前中心点
    neighbors.east = *hash;
    neighbors.west = *hash;
    neighbors.north = *hash;
    neighbors.south = *hash;
    neighbors.south_east = *hash;
    neighbors.south_west = *hash;
    neighbors.north_east = *hash;
    neighbors.north_west = *hash;

    // 计算东、西
    geohash_move_x(&mut neighbors.east, 1);
    geohash_move_y(&mut neighbors.east, 0);

    geohash_move_x(&mut neighbors.west, -1);
    geohash_move_y(&mut neighbors.west, 0);

    // 计算南、北
    geohash_move_x(&mut neighbors.south, 0);
    geohash_move_y(&mut neighbors.south, -1);

    geohash_move_x(&mut neighbors.north, 0);
    geohash_move_y(&mut neighbors.north, 1);

    // 计算四个角
    geohash_move_x(&mut neighbors.north_west, -1);
    geohash_move_y(&mut neighbors.north_west, 1);

    geohash_move_x(&mut neighbors.north_east, 1);
    geohash_move_y(&mut neighbors.north_east, 1);

    geohash_move_x(&mut neighbors.south_east, 1);
    geohash_move_y(&mut neighbors.south_east, -1);

    geohash_move_x(&mut neighbors.south_west, -1);
    geohash_move_y(&mut neighbors.south_west, -1);
}

/// 角度转弧度
#[inline]
fn deg_rad(ang: f64) -> f64 {
    ang * D_R
}

/// 弧度转角度
#[inline]
fn rad_deg(ang: f64) -> f64 {
    ang / D_R
}

// --- 数据结构 ---

/// 形状枚举：支持圆形和矩形搜索
/// Redis C 代码用的是 union，Rust 用 enum 更安全
#[derive(Debug, Clone, Copy)]
pub enum GeoShapeType {
    Circular { radius: f64 },
    Rectangle { height: f64, width: f64 },
}

#[derive(Debug, Clone, Copy)]
pub struct GeoShape {
    pub shape_type: GeoShapeType,
    pub xy: [f64; 2], // 0: 经度, 1: 纬度
    pub conversion: f64, // 单位换算系数
    pub bounds: [f64; 4], // [min_lon, min_lat, max_lon, max_lat]
}

/// 包含中心点和邻居的搜索半径结构
#[derive(Debug, Clone, Copy, Default)]
pub struct GeoHashRadius {
    pub hash: GeoHashBits,
    pub area: GeoHashArea,
    pub neighbors: GeoHashNeighbors,
}

// --- 核心辅助逻辑 (对应 geohash_helper.c) ---

/// 根据搜索半径估算需要的 Geohash 精度 (steps)
pub fn geohash_estimate_steps_by_radius(mut range_meters: f64, lat: f64) -> u8 {
    if range_meters == 0.0 {
        return 26;
    }
    let mut step = 1;
    // 只要范围小于墨卡托最大值，就尝试增加步长（增加精度）
    while range_meters < MERCATOR_MAX {
        range_meters *= 2.0;
        step += 1;
    }
    step -= 2; // 回退两步，确保覆盖

    // 高纬度地区经线收敛，需要特殊处理
    if lat > 66.0 || lat < -66.0 {
        step -= 1;
        if lat > 80.0 || lat < -80.0 {
            step -= 1;
        }
    }

    // 限制范围在 1-26 之间
    if step < 1 { step = 1; }
    if step > 26 { step = 26; }

    step as u8
}

/// 计算搜索形状的边界框 (Bounding Box)
pub fn geohash_bounding_box(shape: &mut GeoShape) -> bool {
    let longitude = shape.xy[0];
    let latitude = shape.xy[1];

    // 计算高和宽
    let (height, width) = match shape.shape_type {
        GeoShapeType::Circular { radius } => (
            shape.conversion * radius,
            shape.conversion * radius
        ),
        GeoShapeType::Rectangle { height, width } => (
            shape.conversion * height / 2.0,
            shape.conversion * width / 2.0
        ),
    };

    // 计算经纬度偏移量
    let lat_delta = rad_deg(height / EARTH_RADIUS_IN_METERS);
    let long_delta_top = rad_deg(width / EARTH_RADIUS_IN_METERS / deg_rad(latitude + lat_delta).cos());
    let long_delta_bottom = rad_deg(width / EARTH_RADIUS_IN_METERS / deg_rad(latitude - lat_delta).cos());

    // 南半球处理：上下方向相反
    let southern_hemisphere = latitude < 0.0;

    shape.bounds[0] = if southern_hemisphere { longitude - long_delta_bottom } else { longitude - long_delta_top };
    shape.bounds[2] = if southern_hemisphere { longitude + long_delta_bottom } else { longitude + long_delta_top };
    shape.bounds[1] = latitude - lat_delta;
    shape.bounds[3] = latitude + lat_delta;

    true
}

/// 计算覆盖指定形状的 Geohash 区域 (九宫格)
/// 这是 GEORADIUS 命令最核心的逻辑
pub fn geohash_calculate_areas_by_shape_wgs84(shape: &mut GeoShape) -> GeoHashRadius {
    // 初始化返回结构
    let mut neighbors = GeoHashNeighbors::default();

    // 计算 Bounding Box
    geohash_bounding_box(shape);
    let min_lon = shape.bounds[0];
    let min_lat = shape.bounds[1];
    let max_lon = shape.bounds[2];
    let max_lat = shape.bounds[3];

    let longitude = shape.xy[0];
    let latitude = shape.xy[1];

    // 计算半径 (米)
    let mut radius_meters = match shape.shape_type {
        GeoShapeType::Circular { radius } => radius,
        GeoShapeType::Rectangle { height, width } => {
            ((width / 2.0).powi(2) + (height / 2.0).powi(2)).sqrt()
        }
    };
    radius_meters *= shape.conversion;

    // 1. 估算精度 step
    let mut steps = geohash_estimate_steps_by_radius(radius_meters, latitude);

    // 获取坐标范围
    let (long_range, lat_range) = geohash_get_coord_range();

    // 现在返回 Option，失败则使用默认值
    let mut hash = geohash_encode(&long_range, &lat_range, longitude, latitude, steps)
        .unwrap_or_default();

    // 计算邻居
    geohash_neighbors(&hash, &mut neighbors);
    
    let mut area = geohash_decode(long_range, lat_range, hash)
        .unwrap_or_default();

    // 3. 检查边界覆盖情况
    let mut decrease_step = false;
    {
        // 注意：decode 接收的是 Copy 类型的 Range，不需要引用
        let north = geohash_decode(long_range, lat_range, neighbors.north).unwrap_or_default();
        let south = geohash_decode(long_range, lat_range, neighbors.south).unwrap_or_default();
        let east = geohash_decode(long_range, lat_range, neighbors.east).unwrap_or_default();
        let west = geohash_decode(long_range, lat_range, neighbors.west).unwrap_or_default();

        if north.latitude.max < max_lat { decrease_step = true; }
        if south.latitude.min > min_lat { decrease_step = true; }
        if east.longitude.max < max_lon { decrease_step = true; }
        if west.longitude.min > min_lon { decrease_step = true; }
    }

    // 如果需要，降低精度并重新计算
    if steps > 1 && decrease_step {
        steps -= 1;
        // 【改动6】重新赋值
        hash = geohash_encode(&long_range, &lat_range, longitude, latitude, steps)
            .unwrap_or_default();

        geohash_neighbors(&hash, &mut neighbors);

        area = geohash_decode(long_range, lat_range, hash)
            .unwrap_or_default();
    }

    // 4. 剔除无效区域 (逻辑不变，只是操作对象变了)
    if steps >= 2 {
        if area.latitude.min < min_lat {
            neighbors.south.zero();
            neighbors.south_west.zero();
            neighbors.south_east.zero();
        }
        if area.latitude.max > max_lat {
            neighbors.north.zero();
            neighbors.north_east.zero();
            neighbors.north_west.zero();
        }
        if area.longitude.min < min_lon {
            neighbors.west.zero();
            neighbors.south_west.zero();
            neighbors.north_west.zero();
        }
        if area.longitude.max > max_lon {
            neighbors.east.zero();
            neighbors.south_east.zero();
            neighbors.north_east.zero();
        }
    }

    // 组装返回结果
    GeoHashRadius {
        hash,
        neighbors,
        area,
    }
}
/// geohash_align_52bits 强制 52位对齐
/// Redis 的 ZSet Score 是 double 类型，只有 52 位尾数精度。
/// 不管当前的 hash.step 是多少（比如只用了 10 位），
/// 都必须把它左移，填满高 52 位，这样生成的整数转成 double 后，
/// 才能在 ZSet 里保持正确的字典序排列。
pub fn geohash_align_52bits(hash: GeoHashBits) -> u64 {
    let mut bits = hash.bits;
    bits <<= 52 - hash.step * 2;
    bits
}

// --- 距离计算函数 (Haversine 公式) ---

/// 计算纬度距离
pub fn geohash_get_lat_distance(lat1d: f64, lat2d: f64) -> f64 {
    EARTH_RADIUS_IN_METERS * (deg_rad(lat2d) - deg_rad(lat1d)).abs()
}

/// 计算两点间的球面距离 (Haversine)
pub fn geohash_get_distance(lon1d: f64, lat1d: f64, lon2d: f64, lat2d: f64) -> f64 {
    let lon1r = deg_rad(lon1d);
    let lon2r = deg_rad(lon2d);
    let v = ((lon2r - lon1r) / 2.0).sin();

    // 如果经度几乎相同，退化为纬度距离计算，避免昂贵的 math 运算
    if v == 0.0 {
        return geohash_get_lat_distance(lat1d, lat2d);
    }

    let lat1r = deg_rad(lat1d);
    let lat2r = deg_rad(lat2d);
    let u = ((lat2r - lat1r) / 2.0).sin();

    let a = u * u + lat1r.cos() * lat2r.cos() * v * v;
    2.0 * EARTH_RADIUS_IN_METERS * a.sqrt().asin()
}

/// 判断点是否在半径内，并返回距离
pub fn geohash_get_distance_if_in_radius(
    x1: f64, y1: f64,
    x2: f64, y2: f64,
    radius: f64,
    distance: &mut f64
) -> bool {
    *distance = geohash_get_distance(x1, y1, x2, y2);
    if *distance > radius {
        return false;
    }
    true
}

/// WGS84 版本的封装
pub fn geohash_get_distance_if_in_radius_wgs84(
    x1: f64, y1: f64,
    x2: f64, y2: f64,
    radius: f64,
    distance: &mut f64
) -> bool {
    geohash_get_distance_if_in_radius(x1, y1, x2, y2, radius, distance)
}

/// 判断点是否在矩形内
pub fn geohash_get_distance_if_in_rectangle(
    width_m: f64, height_m: f64,
    x1: f64, y1: f64,
    x2: f64, y2: f64,
    distance: &mut f64
) -> bool {
    // 优先检查纬度距离（计算成本低）
    let lat_distance = geohash_get_lat_distance(y2, y1);
    if lat_distance > height_m / 2.0 {
        return false;
    }
    // 检查经度距离
    let lon_distance = geohash_get_distance(x2, y2, x1, y2);
    if lon_distance > width_m / 2.0 {
        return false;
    }
    *distance = geohash_get_distance(x1, y1, x2, y2);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beijing_encoding_compatibility() {
        let lon = 116.40;
        let lat = 39.90;

        // 1. 编码
        let hash = geohash_encode_wgs84(lon, lat, GEO_STEP_MAX).expect("Encoding failed");

        // 2. 对齐
        let score_int = geohash_align_52bits(hash);
        let redis_expected_score: u64 = 4069885360207904;

        assert_eq!(score_int, redis_expected_score);
    }

    #[test]
    fn test_decoding() {
        let redis_score: u64 = 4069885360207904;
        let hash = GeoHashBits {
            bits: redis_score >> (52 - GEO_STEP_MAX * 2),
            step: GEO_STEP_MAX,
        };

        // 解码
        let xy = geohash_decode_to_long_lat_wgs84(hash).expect("Decoding failed");

        println!("Decoded: Lon {}, Lat {}", xy[0], xy[1]);
        assert!((xy[0] - 116.40).abs() < 0.0001);
        assert!((xy[1] - 39.90).abs() < 0.0001);
    }
}