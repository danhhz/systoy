// Copyright 2021 Daniel Harrison. All Rights Reserved.

use std::array::IntoIter;
use std::io::Write;
use std::iter;
use std::time::Instant;

use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput};

fn buf_sizes() -> Vec<usize> {
    iter::successors(Some(1usize), |x| Some(*x * 2)).take_while(|x| *x <= 32 * 1024).collect()
}

fn buf_data(size_bytes: usize) -> Vec<u8> {
    iter::successors(Some(0u64), |x| Some(*x + 1))
        .flat_map(|x| IntoIter::new(x.to_le_bytes()))
        .take(size_bytes)
        .collect()
}

fn serial_run(b: &mut Bencher<WallTime>, buf_size: &usize) {
    let data = buf_data(*buf_size);
    let mut f = tempfile::tempfile().unwrap();
    b.iter_custom(|iters| {
        let start = Instant::now();
        for _i in 0..iters {
            f.write_all(&data).unwrap();
            f.sync_data().unwrap();
        }
        start.elapsed()
    });
}

fn serial(c: &mut Criterion) {
    let mut group = c.benchmark_group("serial");
    for buf_size in buf_sizes().into_iter() {
        group.throughput(Throughput::Bytes(buf_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(buf_size), &buf_size, serial_run);
    }
    group.finish();
}

criterion_group!(benches, serial);
criterion_main!(benches);
