use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rudis_server::store::geo::geohash::{geohash_encode_wgs84, GEO_STEP_MAX};
use rudis_server::store::geo::{Geo, GeoUnit, GeoRadiusOptions};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("geohash encode", |b| {
        b.iter(|| {
            // black_box 防止编译器优化
            geohash_encode_wgs84(
                black_box(116.40),
                black_box(39.90),
                black_box(GEO_STEP_MAX)
            )
        })
    });
}

// 构造一个填充了 N 个点的引擎
fn setup_engine(count: usize) -> Geo {
    let mut geo = Geo::new();
    let mut rng = rand::thread_rng();

    // 模拟北京范围内的密集点
    for i in 0..count {
        let lon = 116.0 + rng.gen::<f64>(); // 116.0 ~ 117.0
        let lat = 39.0 + rng.gen::<f64>();  // 39.0 ~ 40.0
        let name = format!("member_{}", i);
        geo.add(name, lon, lat).unwrap();
    }
    geo
}

fn bench_georadius(c: &mut Criterion) {
    let mut group = c.benchmark_group("GeoEngine");

    // 1. 准备数据：10万个点 (这在内存数据库里算中等规模)
    let engine = setup_engine(100_000);

    // 2. 也是北京范围内的随机查询点
    let query_lon = 116.5;
    let query_lat = 39.5;

    // 测试场景：搜索 5km 半径
    group.bench_function("radius_5km_100k_points", |b| {
        b.iter(|| {
            // 使用 black_box 防止编译器优化掉代码
            let res = engine.radius(
                black_box(query_lon),
                black_box(query_lat),
                black_box(5.0),
                black_box(GeoUnit::Kilometers),
                black_box(&GeoRadiusOptions::default())
            );
            // 确保结果被使用
            black_box(res.len());
        })
    });

    // 测试场景：搜索 500m 半径 (更精细的 Range Scan)
    group.bench_function("radius_500m_100k_points", |b| {
        b.iter(|| {
            let res = engine.radius(
                black_box(query_lon),
                black_box(query_lat),
                black_box(500.0),
                black_box(GeoUnit::Meters),
                black_box(&GeoRadiusOptions::default())
            );
            black_box(res.len());
        })
    });

    group.finish();
}

fn bench_geoadd(c: &mut Criterion) {
    let mut group = c.benchmark_group("GeoEngine_Write");
    let mut engine = Geo::new();
    let mut i = 0;

    group.bench_function("add_single_point", |b| {
        b.iter(|| {
            i += 1;
            engine.add(format!("u_{}", i), 116.4, 39.9).unwrap();
        })
    });
    group.finish();
}


criterion_group!(benches, criterion_benchmark, bench_georadius, bench_geoadd);
criterion_main!(benches);