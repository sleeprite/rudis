//! Geo 模块全方位系统测试
//!
//! 测试策略：
//! 1. Functional: 基础功能验证
//! 2. Consistency: 索引一致性验证 (Move语义)
//! 3. Collision: 哈希碰撞验证
//! 4. Search: 复杂搜索场景
//! 5. Extension: GeoJSON 与属性
//! 6. Persistence: 持久化验证
//! 7. EdgeCases: 边界值

#[cfg(test)]
mod tests {
    use super::super::*;

    // 浮点数比较误差容忍度
    const EPSILON: f64 = 1e-4;

    fn assert_f64_eq(a: f64, b: f64) {
        assert!((a - b).abs() < EPSILON, "Expected {}, got {}", a, b);
    }

    // --- 1. 基础 CRUD 功能测试 ---
    #[test]
    fn test_basic_crud_lifecycle() {
        let mut geo = Geo::new();

        // 1. Add
        assert_eq!(geo.add("Beijing".to_string(), 116.40, 39.90).unwrap(), 1); // New
        assert_eq!(geo.len(), 1);

        // 2. Overwrite (Update)
        // 模拟位置微调
        assert_eq!(geo.add("Beijing".to_string(), 116.41, 39.91).unwrap(), 0); // Update
        assert_eq!(geo.len(), 1); // 数量不应增加

        // 3. Pos
        let (lon, lat) = geo.pos("Beijing").unwrap();
        assert_f64_eq(lon, 116.41);
        assert_f64_eq(lat, 39.91);

        // 4. Dist
        // 添加第二个点用于测距
        geo.add("Shanghai".to_string(), 121.47, 31.23).unwrap();
        let dist = geo.dist("Beijing", "Shanghai", GeoUnit::Kilometers).unwrap();
        assert!(dist > 1060.0 && dist < 1070.0);

        // 5. Hash
        let hash = geo.hash("Beijing").unwrap();
        assert_eq!(hash.len(), 11); // Redis 标准是 11 字符 Base32

        // 6. Remove
        assert_eq!(geo.rem("Beijing"), 1);
        assert!(geo.pos("Beijing").is_none());
        assert_eq!(geo.len(), 1); // 剩下一个 Shanghai
    }

    // --- 2. 索引一致性测试 (核心) ---
    // 验证：当一个物体移动很远（跨越 Geohash 格子）时，旧格子里的数据是否被清理
    #[test]
    fn test_index_consistency_on_move() {
        let mut geo = Geo::new();
        let name = "DeliveryGuy";

        // 1. 初始位置：A点 (116.0, 39.0)
        geo.add(name.to_string(), 116.0, 39.0).unwrap();

        // 确保 A 点附近能搜到
        let res_a = geo.radius(116.0, 39.0, 1.0, GeoUnit::Kilometers, &GeoRadiusOptions::default());
        assert_eq!(res_a.len(), 1);
        assert_eq!(res_a[0].name, name);

        // 2. 移动到：B点 (117.0, 40.0) - 距离很远，Geohash 肯定变了
        geo.add(name.to_string(), 117.0, 40.0).unwrap();

        // 3. 验证 B 点附近能搜到
        let res_b = geo.radius(117.0, 40.0, 1.0, GeoUnit::Kilometers, &GeoRadiusOptions::default());
        assert_eq!(res_b.len(), 1);
        assert_eq!(res_b[0].name, name);

        // 4. 关键验证：A 点附近必须搜不到！(验证 remove_from_spatial 是否生效)
        let res_ghost = geo.radius(116.0, 39.0, 1.0, GeoUnit::Kilometers, &GeoRadiusOptions::default());
        assert_eq!(res_ghost.len(), 0, "Found ghost data at old location!");
    }

    // --- 3. 哈希碰撞测试 (Bucket 机制) ---
    // 验证：同一个坐标（或极近坐标）的多个不同 Member 是否能共存
    #[test]
    fn test_hash_collision_handling() {
        let mut geo = Geo::new();

        // 两人在完全相同的坐标
        geo.add("UserA".to_string(), 100.0, 50.0).unwrap();
        geo.add("UserB".to_string(), 100.0, 50.0).unwrap();
        // UserC 在极近的坐标 (Geohash u64 应该相同)
        geo.add("UserC".to_string(), 100.0, 50.00000001).unwrap();

        assert_eq!(geo.len(), 3);

        // 搜索该点
        let opts = GeoRadiusOptions {
            count: None,
            ..Default::default()
        };
        let res = geo.radius(100.0, 50.0, 10.0, GeoUnit::Meters, &opts);

        assert_eq!(res.len(), 3);
        let names: Vec<String> = res.iter().map(|r| r.name.clone()).collect();
        assert!(names.contains(&"UserA".to_string()));
        assert!(names.contains(&"UserB".to_string()));
        assert!(names.contains(&"UserC".to_string()));
    }

    // --- 4. 复杂搜索场景测试 ---
    #[test]
    fn test_georadius_sorting_and_limiting() {
        let mut geo = Geo::new();
        // 构造一条线上的点：
        // Center (0,0) -> A(10km) -> B(20km) -> C(30km)
        // 0.1 度纬度 ≈ 11.1 km
        geo.add("Center".to_string(), 0.0, 0.0).unwrap();
        geo.add("PointA".to_string(), 0.0, 0.1).unwrap();  // ~11km
        geo.add("PointB".to_string(), 0.0, 0.2).unwrap();  // ~22km
        geo.add("PointC".to_string(), 0.0, 0.3).unwrap();  // ~33km

        // 1. ASC 排序 (由近到远)
        let opts_asc = GeoRadiusOptions {
            sort_asc: true,
            withdist: true,
            ..Default::default()
        };
        let res_asc = geo.radius(0.0, 0.0, 100.0, GeoUnit::Kilometers, &opts_asc);
        assert_eq!(res_asc[0].name, "Center");
        assert_eq!(res_asc[1].name, "PointA");
        assert_eq!(res_asc[2].name, "PointB");
        assert_eq!(res_asc[3].name, "PointC");

        // 2. DESC 排序 (由远到近)
        let opts_desc = GeoRadiusOptions {
            sort_desc: true,
            withdist: true,
            ..Default::default()
        };
        let res_desc = geo.radius(0.0, 0.0, 100.0, GeoUnit::Kilometers, &opts_desc);
        assert_eq!(res_desc[0].name, "PointC");

        // 3. Count 限制
        let opts_limit = GeoRadiusOptions {
            count: Some(2),
            sort_asc: true,
            ..Default::default()
        };
        let res_limit = geo.radius(0.0, 0.0, 100.0, GeoUnit::Kilometers, &opts_limit);
        assert_eq!(res_limit.len(), 2);
        assert_eq!(res_limit[1].name, "PointA");
    }

    #[test]
    fn test_georadius_options_response() {
        let mut geo = Geo::new();
        geo.add("Target".to_string(), 10.0, 20.0).unwrap();

        let opts = GeoRadiusOptions {
            withcoord: true,
            withdist: true,
            withhash: true,
            ..Default::default()
        };

        let res = geo.radius(10.0, 20.0, 1.0, GeoUnit::Meters, &opts);
        let item = &res[0];

        // 验证坐标回显
        assert_f64_eq(item.longitude, 10.0);
        assert_f64_eq(item.latitude, 20.0);

        // 验证距离 (自己搜自己应该是 0)
        assert_f64_eq(item.distance, 0.0);

        // 验证 Hash (Redis 52-bit 格式，不应为0)
        assert!(item.hash > 0);
    }

    // --- 5. GeoJSON 与属性测试 ---
    #[test]
    fn test_add_from_geojson() {
        let mut geo = Geo::new();

        // 模拟一个带属性的 GeoJSON Feature
        let json = r#"{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [116.5, 39.5]
            },
            "properties": {
                "name": "GasStation_1",
                "price": 8.5,
                "open_24h": true
            }
        }"#;

        assert_eq!(geo.add_from_geo_json(json).unwrap(), 1);

        // 验证基本坐标
        let (lon, lat) = geo.pos("GasStation_1").unwrap();
        assert_f64_eq(lon, 116.5);
        assert_f64_eq(lat, 39.5);

        // 验证属性 (由于 Geo 内部对 GeoPoint 进行了封装，
        // 目前 pos() 只返回坐标。如果要验证 properties，
        // 需要通过内部访问或后续扩展 GEODETAILS 命令。
        // 这里主要验证写入流程不崩，且 member_index 正确建立)
        assert!(geo.pos("GasStation_1").is_some());
    }

    #[test]
    fn test_geojson_invalid_input() {
        let mut geo = Geo::new();
        // 错误的 JSON
        assert!(geo.add_from_geo_json(r#"{ "type": "Invalid" }"#).is_err());
        // 错误的几何类型 (Polygon 暂不支持写入点索引)
        let polygon_json = r#"{
            "type": "Feature",
            "geometry": {
                "type": "Polygon",
                "coordinates": [[[0,0], [1,0], [1,1], [0,1], [0,0]]]
            },
            "properties": {}
        }"#;
        assert!(geo.add_from_geo_json(polygon_json).is_err());
    }

    // --- 6. 持久化测试 (Bincode) ---
    #[test]
    fn test_persistence_roundtrip() {
        use bincode::{encode_to_vec, decode_from_slice, config};

        let mut original = Geo::new();
        original.add("PersistPoint".to_string(), 123.45, 67.89).unwrap();
        original.add("AnotherPoint".to_string(), -10.0, -20.0).unwrap();

        // Encode
        let config = config::standard();
        let encoded = encode_to_vec(&original, config).unwrap();

        // Decode
        let (decoded, _): (Geo, usize) = decode_from_slice(&encoded, config).unwrap();

        // Verify consistency
        assert_eq!(original.len(), decoded.len());

        let (lon, lat) = decoded.pos("PersistPoint").unwrap();
        assert_f64_eq(lon, 123.45);
        assert_f64_eq(lat, 67.89);

        let (lon, _lat) = decoded.pos("AnotherPoint").unwrap();
        assert_f64_eq(lon, -10.0);
    }

    // --- 7. 边界与异常测试 ---
    #[test]
    fn test_boundary_coordinates() {
        let mut geo = Geo::new();

        // 极北
        geo.add("NorthPole".to_string(), 0.0, 85.05).unwrap();
        // 极南
        geo.add("SouthPole".to_string(), 0.0, -85.05).unwrap();
        // 跨日界线东
        geo.add("EastEnd".to_string(), 180.0, 0.0).unwrap();
        // 跨日界线西
        geo.add("WestEnd".to_string(), -180.0, 0.0).unwrap();

        assert_eq!(geo.len(), 4);

        // 搜索极北附近
        let res = geo.radius(0.0, 85.0, 100.0, GeoUnit::Kilometers, &GeoRadiusOptions::default());
        assert!(res.len() >= 1);
        assert_eq!(res[0].name, "NorthPole");
    }

    #[test]
    fn test_invalid_coordinates() {
        let mut geo = Geo::new();
        // 纬度超标
        assert!(geo.add("BadLat".to_string(), 0.0, 91.0).is_err());
        // 经度超标
        assert!(geo.add("BadLon".to_string(), 181.0, 0.0).is_err());
        // 正常
        assert!(geo.add("Good".to_string(), 0.0, 0.0).is_ok());
    }

    #[test]
    fn test_empty_engine() {
        let geo = Geo::new();
        assert_eq!(geo.len(), 0);
        assert!(geo.pos("Nobody").is_none());
        assert!(geo.hash("Nobody").is_none());

        let res = geo.radius(0.0, 0.0, 100.0, GeoUnit::Meters, &GeoRadiusOptions::default());
        assert!(res.is_empty());
    }
}