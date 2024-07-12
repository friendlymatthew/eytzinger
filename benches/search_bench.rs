#![feature(core_intrinsics)]

use std::cmp::Ordering;
use std::intrinsics::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use eytzinger::EytzingerVec;

fn binary_search<T: Ord + Copy>(data: &[T], x: &T) -> usize {
    let mut hi = data.len() - 1;
    let mut lo = 0;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        match data[mid].cmp(x) {
            Ordering::Equal => return mid,
            Ordering::Greater => hi = mid - 1,
            Ordering::Less => lo = mid + 1,
        }
    }

    0
}

fn eytzinger_search_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Eytzinger Search");

    for &size in &[1000, 10_000, 100_000] {
        let data: Vec<i32> = (0..size).collect();
        let eytzinger_vec = EytzingerVec::from_slice(&data);

        let needle = data[(size / 2) as usize];

        group.bench_with_input(BenchmarkId::new("Eytzinger", size), &data, |b, data| {
            b.iter(|| eytzinger_vec.search(black_box(&needle)));
        });

        group.bench_with_input(
            BenchmarkId::new("Rust Binary Search", size),
            &data,
            |b, data| {
                b.iter(|| data.binary_search(black_box(&needle)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Naive Binary Search", size),
            &data,
            |b, data| {
                b.iter(|| binary_search(data, black_box(&needle)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, eytzinger_search_bench);
criterion_main!(benches);
