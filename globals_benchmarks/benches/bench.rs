use criterion::{black_box, criterion_group, criterion_main, Criterion};
use globals_benchmarks::*;

const ITERATIONS: usize = 10_000;

pub fn mutable_benchmark(c: &mut Criterion) {
    c.bench_function("tls 1", |b| b.iter(|| mutable_globals::tls(black_box(ITERATIONS))));
    c.bench_function("tls_2", |b| b.iter(|| mutable_globals::tls_2(black_box(ITERATIONS))));
    c.bench_function("tls_3", |b| b.iter(|| mutable_globals::tls_3(black_box(ITERATIONS))));
    
    
    c.bench_function("std_once 1", |b| b.iter(|| mutable_globals::std_once(black_box(ITERATIONS))));
    c.bench_function("std_once 2", |b| b.iter(|| mutable_globals::std_once_2(black_box(ITERATIONS))));
    c.bench_function("std_once 3", |b| b.iter(|| mutable_globals::std_once_3(black_box(ITERATIONS))));
    
    c.bench_function("lazy_static 1", |b| b.iter(|| mutable_globals::lazy_static(black_box(ITERATIONS))));
    c.bench_function("lazy_static 2", |b| b.iter(|| mutable_globals::lazy_static_2(black_box(ITERATIONS))));
    c.bench_function("lazy_static 3", |b| b.iter(|| mutable_globals::lazy_static_3(black_box(ITERATIONS))));
    
    c.bench_function("once_cell 1", |b| b.iter(|| mutable_globals::once_cell(black_box(ITERATIONS))));
    c.bench_function("once_cell 2", |b| b.iter(|| mutable_globals::once_cell_2(black_box(ITERATIONS))));
    c.bench_function("once_cell 3", |b| b.iter(|| mutable_globals::once_cell_3(black_box(ITERATIONS))));
    
    c.bench_function("atomic 1", |b| b.iter(|| mutable_globals::atomic(black_box(ITERATIONS))));
    c.bench_function("atomic 2", |b| b.iter(|| mutable_globals::atomic_2(black_box(ITERATIONS))));
    c.bench_function("atomic 3", |b| b.iter(|| mutable_globals::atomic_3(black_box(ITERATIONS))));

    c.bench_function("atomic_seq 1", |b| b.iter(|| mutable_globals::atomic_seq(black_box(ITERATIONS))));
    c.bench_function("atomic_seq 2", |b| b.iter(|| mutable_globals::atomic_seq_2(black_box(ITERATIONS))));
    c.bench_function("atomic_seq 3", |b| b.iter(|| mutable_globals::atomic_seq_3(black_box(ITERATIONS))));
}

pub fn ro_benchmark(c: &mut Criterion) {
    c.bench_function("tls 1", |b| b.iter(|| read_only_globals::tls(black_box(ITERATIONS))));
    c.bench_function("tls_2", |b| b.iter(|| read_only_globals::tls_2(black_box(ITERATIONS))));
    c.bench_function("tls_3", |b| b.iter(|| read_only_globals::tls_3(black_box(ITERATIONS))));
    
    
    c.bench_function("std_once 1", |b| b.iter(|| read_only_globals::std_once(black_box(ITERATIONS))));
    c.bench_function("std_once 2", |b| b.iter(|| read_only_globals::std_once_2(black_box(ITERATIONS))));
    c.bench_function("std_once 3", |b| b.iter(|| read_only_globals::std_once_3(black_box(ITERATIONS))));
    
    c.bench_function("lazy_static 1", |b| b.iter(|| read_only_globals::lazy_static(black_box(ITERATIONS))));
    c.bench_function("lazy_static 2", |b| b.iter(|| read_only_globals::lazy_static_2(black_box(ITERATIONS))));
    c.bench_function("lazy_static 3", |b| b.iter(|| read_only_globals::lazy_static_3(black_box(ITERATIONS))));
    
    c.bench_function("once_cell 1", |b| b.iter(|| read_only_globals::once_cell(black_box(ITERATIONS))));
    c.bench_function("once_cell 2", |b| b.iter(|| read_only_globals::once_cell_2(black_box(ITERATIONS))));
    c.bench_function("once_cell 3", |b| b.iter(|| read_only_globals::once_cell_3(black_box(ITERATIONS))));
    
    c.bench_function("atomic 1", |b| b.iter(|| read_only_globals::atomic(black_box(ITERATIONS))));
    c.bench_function("atomic 2", |b| b.iter(|| read_only_globals::atomic_2(black_box(ITERATIONS))));
    c.bench_function("atomic 3", |b| b.iter(|| read_only_globals::atomic_3(black_box(ITERATIONS))));

    c.bench_function("atomic_seq 1", |b| b.iter(|| read_only_globals::atomic_seq(black_box(ITERATIONS))));
    c.bench_function("atomic_seq 2", |b| b.iter(|| read_only_globals::atomic_seq_2(black_box(ITERATIONS))));
    c.bench_function("atomic_seq 3", |b| b.iter(|| read_only_globals::atomic_seq_3(black_box(ITERATIONS))));
}

criterion_group!(benches, mutable_benchmark, ro_benchmark);
criterion_main!(benches);
