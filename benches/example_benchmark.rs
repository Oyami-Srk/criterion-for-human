use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 0..n {
        let temp = a;
        a = b;
        b += temp;
    }
    a
}

pub fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    for size in [10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", size), size, |b, &size| {
            b.iter(|| fibonacci_recursive(black_box(size)));
        });

        group.bench_with_input(BenchmarkId::new("Iterative", size), size, |b, &size| {
            b.iter(|| fibonacci_iterative(black_box(size)));
        });
    }

    group.finish();
}

pub fn string_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Operations");

    group.bench_function("Concat/Small (10 bytes)", |b| {
        b.iter(|| {
            let s = String::from("Hello");
            black_box(s + "World")
        });
    });

    group.bench_function("Concat/Large (1KB)", |b| {
        let large_str = "x".repeat(512);
        b.iter(|| {
            let s = String::from(&large_str);
            black_box(s + &large_str)
        });
    });

    group.finish();
}

criterion_group!(benches, fibonacci_benchmark, string_benchmark);
criterion_main!(benches);
