use std::hint::black_box;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use json_deserialize_perf::deser_reader::AssetPreloader as AssetPreloader;
use json_deserialize_perf::deser_reader_simd::AssetPreloader as SimdAssetPreloader;
use json_deserialize_perf::deser_simd_alloc_buff_in_one_go::AssetPreloader as SimdAllocAllAssetPreloader;
use json_deserialize_perf::deser_alloc_buff_in_one_go::AssetPreloader as AllocAllAssetPreloader;

fn simd_alloc_all_bench_asset_preloader(c: &mut Criterion) {
    c.bench_function("SimdAllocAllAssetPreloader", |b| {
        b.iter(|| {
            let preloader = SimdAllocAllAssetPreloader::new();
        })
    });
}

fn alloc_all_bench_asset_preloader(c: &mut Criterion) {
    c.bench_function("AllocAllAssetPreloader", |b| {
        b.iter(|| {
            let preloader = AllocAllAssetPreloader::new();
        })
    });
}

fn simd_bench_asset_preloader(c: &mut Criterion) {
    c.bench_function("SimdAssetPreloader", |b| {
        b.iter(|| {
            let preloader = SimdAssetPreloader::new();
        })
    });
}

fn bench_asset_preloader(c: &mut Criterion) {
    c.bench_function("AssetPreloader", |b| {
        b.iter(|| {
            let preloader = AssetPreloader::new();
        })
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(30))
        .sample_size(10)
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = simd_alloc_all_bench_asset_preloader,
              alloc_all_bench_asset_preloader,
              simd_bench_asset_preloader,
              bench_asset_preloader,
}
criterion_main!(benches);