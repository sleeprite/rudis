//! Geo 单元测试

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_geoadd_and_geopos() {
        let mut engine = Geo::new();
        engine.add("beijing".to_string(), 116.3974, 39.9093).unwrap();
        engine.add("shanghai".to_string(), 121.4737, 31.2304).unwrap();

        let (lon, lat) = engine.pos("beijing").unwrap();
        assert!((lon - 116.3974).abs() < 0.0001);
        assert!((lat - 39.9093).abs() < 0.0001);

        let (lon, lat) = engine.pos("shanghai").unwrap();
        assert!((lon - 121.4737).abs() < 0.0001);
        assert!((lat - 31.2304).abs() < 0.0001);

        assert!(engine.pos("nonexistent").is_none());
    }

    #[test]
    fn test_geodist() {
        let mut engine = Geo::new();
        engine.add("beijing".to_string(), 116.3974, 39.9093).unwrap();
        engine.add("shanghai".to_string(), 121.4737, 31.2304).unwrap();

        let dist_km = engine.dist("beijing", "shanghai", GeoUnit::Kilometers).unwrap();
        // 北京到上海约 1068 km
        assert!(dist_km > 1000.0 && dist_km < 1100.0);
    }

    #[test]
    fn test_geohash() {
        let mut engine = Geo::new();
        engine.add("beijing".to_string(), 116.3974, 39.9093).unwrap();

        let hash = engine.hash("beijing").unwrap();
        assert_eq!(hash.len(), 11);
        assert!(hash.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_georadius() {
        let mut engine = Geo::new();
        engine.add("tiananmen".to_string(), 116.3974, 39.9093).unwrap();
        engine.add("forbidden_city".to_string(), 116.3972, 39.9163).unwrap();
        engine.add("shanghai".to_string(), 121.4737, 31.2304).unwrap();

        let options = GeoRadiusOptions {
            withdist: true,
            ..Default::default()
        };
        let results = engine.radius(116.3974, 39.9093, 5.0, GeoUnit::Kilometers, &options);
        // 天安门和故宫应在 5km 内，上海不在
        assert!(results.len() >= 2);
        let names: Vec<_> = results.iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"tiananmen"));
        assert!(names.contains(&"forbidden_city"));
        assert!(!names.contains(&"shanghai"));
    }

    #[test]
    fn test_georem() {
        let mut engine = Geo::new();
        engine.add("beijing".to_string(), 116.3974, 39.9093).unwrap();
        assert_eq!(engine.len(), 1);

        let removed = engine.rem("beijing");
        assert_eq!(removed, 1);
        assert_eq!(engine.len(), 0);
        assert!(engine.pos("beijing").is_none());

        let removed = engine.rem("nonexistent");
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_add_overwrite() {
        let mut engine = Geo::new();
        let r1 = engine.add("beijing".to_string(), 116.0, 39.0).unwrap();
        assert_eq!(r1, 1);
        let r2 = engine.add("beijing".to_string(), 121.0, 31.0).unwrap();
        assert_eq!(r2, 0); // 覆盖已存在

        let (lon, lat) = engine.pos("beijing").unwrap();
        assert!((lon - 121.0).abs() < 0.0001);
    }
}
