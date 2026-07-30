use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::RngExt;
use rank_up::algorithms::selection_sort::selection_sort;

fn bench_without_generation_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting");

    let size = 10_000;

    group.bench_with_input(BenchmarkId::new("Vec-Sort", size), &size, |b, &s| {
        b.iter_with_setup(
            || {
                let mut rng = rand::rng();
                let mut data: Vec<i32> = Vec::with_capacity(s);
                for _ in 0..s {
                    data.push(rng.random());
                }
                data
            },
            |mut data| {
                selection_sort(black_box(&mut data));
            },
        );
    });
    group.finish();
}

criterion_group!(benches, bench_without_generation_overhead);
criterion_main!(benches);
