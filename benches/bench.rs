use criterion::{criterion_group, criterion_main, Criterion};
use fastmerkle_rust::{hasher::FastHasherPool, merkle_tree::generate_merkle_tree};
use rand::{thread_rng, Rng};
use std::sync::Arc;

fn generate_random_data(size: usize) -> Vec<Vec<u8>> {
    let mut random_data = Vec::with_capacity(size);

    for _ in 0..size {
        let mut data = vec![0u8; 32];
        thread_rng().fill(&mut data[..]);
        random_data.push(data);
    }
    random_data
}

fn benchmark_generate_merkle_tree(c: &mut Criterion, size: usize) {
    let input_data = generate_random_data(size);
    let hasher_pool = Arc::new(FastHasherPool::new(size));

    c.bench_function(&format!("Generate Merkle Tree {}", size), |b| {
        b.iter(|| {
            generate_merkle_tree(input_data.clone(), Arc::clone(&hasher_pool)).unwrap();
        })
    });
}

fn benchmark_generate_merkle_tree_5(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 5);
}
fn benchmark_generate_merkle_tree_50(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 50);
}
fn benchmark_generate_merkle_tree_500(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 500);
}
fn benchmark_generate_merkle_tree_1_000(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 1_000);
}
fn benchmark_generate_merkle_tree_10_000(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 10_000);
}
fn benchmark_generate_merkle_tree_1_000_000(c: &mut Criterion) {
    benchmark_generate_merkle_tree(c, 1_000_000);
}

criterion_group!(
    benches,
    benchmark_generate_merkle_tree_5,
    benchmark_generate_merkle_tree_50,
    benchmark_generate_merkle_tree_500,
    benchmark_generate_merkle_tree_1_000,
    // benchmark_generate_merkle_tree_10_000,
    // benchmark_generate_merkle_tree_1_000_000,
);
criterion_main!(benches);
