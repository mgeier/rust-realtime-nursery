use criterion::{black_box, criterion_group, criterion_main};

pub fn criterion_benchmark(criterion: &mut criterion::Criterion) {
    let mut group = criterion.benchmark_group("std-allocation");

    //group.sampling_mode(criterion::SamplingMode::Flat);
    //group.sample_size(1000);

    group.bench_function("vec-1-byte", |b| {
        b.iter(|| {
            let _v = black_box(Vec::<u8>::with_capacity(black_box(1)));

            // NB: Vec is deallocated here
        })
    });

    group.bench_function("vec-1000-byte", |b| {
        b.iter(|| {
            let _v = black_box(Vec::<u8>::with_capacity(black_box(1000)));

            // NB: Vec is deallocated here
        })
    });

    let mut v = Vec::<u8>::new();

    group.bench_function("vec-push-1-byte", |b| {
        b.iter(|| {
            v.push(black_box(99));
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
