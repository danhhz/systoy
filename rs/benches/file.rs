// Copyright 2021 Daniel Harrison. All Rights Reserved.

use std::array::IntoIter;
use std::io::Write;
use std::iter;
use std::time::Instant;

use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, AxisScale, Bencher, BenchmarkId, Criterion, PlotConfiguration,
    Throughput,
};

fn interesting_buf_sizes() -> Vec<usize> {
    vec![
        1,           // Worst case
        4 * 1024,    // Disk page size,
        1024 * 1024, // Big-ish write
    ]
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

fn sync_at_end_run(b: &mut Bencher<WallTime>, buf_size: &usize) {
    let data = buf_data(*buf_size);
    let mut f = tempfile::tempfile().unwrap();
    b.iter_custom(|iters| {
        let start = Instant::now();
        for _i in 0..iters {
            f.write_all(&data).unwrap();
        }
        f.sync_data().unwrap();
        start.elapsed()
    });
}

fn file_log_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_log_append");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for buf_size in interesting_buf_sizes().into_iter() {
        group.throughput(Throughput::Bytes(buf_size as u64));
        group.bench_with_input(BenchmarkId::new("serial", buf_size), &buf_size, serial_run);
        group.bench_with_input(
            BenchmarkId::new("sync_at_end", buf_size),
            &buf_size,
            sync_at_end_run,
        );
    }
    group.finish();
}

criterion_group!(benches, file_log_append);
criterion_main!(benches);
