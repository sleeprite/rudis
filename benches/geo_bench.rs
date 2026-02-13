use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rudis_server::store::geo::geohash::{geohash_encode_wgs84, GEO_STEP_MAX};

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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);