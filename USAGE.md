# Usage Guide for criterion-for-human

## Quick Start

```bash
# Install the tool
cargo install --path .

# Run benchmarks and generate markdown
cargo criterion --message-format=json | criterion-for-human > BENCHMARKS.md
```

## Complete Workflow

### 1. Write Benchmarks

Create a benchmark file in `benches/` directory:

```rust
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn my_function(n: u64) -> u64 {
    // Your code here
    n * 2
}

pub fn my_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("MyBenchmarks");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("MyFunction", size), 
            size, 
            |b, &size| {
                b.iter(|| my_function(black_box(size)));
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
```

### 2. Add Criterion to Cargo.toml

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

### 3. Run Benchmarks

First time (creates baseline):
```bash
cargo criterion --message-format=json > benchmarks.json
```

Subsequent runs (compares against baseline):
```bash
cargo criterion --message-format=json > benchmarks.json
```

### 4. Generate Markdown Report

```bash
cat benchmarks.json | criterion-for-human > BENCHMARKS.md
```

Or in a single command:
```bash
cargo criterion --message-format=json | criterion-for-human > BENCHMARKS.md
```

## Benchmark ID Structure

The tool groups benchmarks by their ID structure. Use the format:

```
<Group Name>/<Benchmark Name>/<Parameter>
```

### Example 1: Simple Function Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // ID: "Math Operations/Addition/Simple"
    c.bench_function("Math Operations/Addition/Simple", |b| {
        b.iter(|| black_box(1 + 1));
    });
    
    // ID: "Math Operations/Multiplication/Simple"
    c.bench_function("Math Operations/Multiplication/Simple", |b| {
        b.iter(|| black_box(2 * 2));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

This creates a table named "Math Operations" with benchmarks "Addition/Simple" and "Multiplication/Simple".

### Example 2: Parameterized Benchmarks

```rust
use criterion::{black_box, BenchmarkId, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    
    for size in [10, 100, 1000].iter() {
        // ID: "Sorting Algorithms/QuickSort/<size>"
        group.bench_with_input(
            BenchmarkId::new("QuickSort", size),
            size,
            |b, &size| {
                b.iter(|| quick_sort(black_box(generate_vec(size))));
            }
        );
        
        // ID: "Sorting Algorithms/MergeSort/<size>"
        group.bench_with_input(
            BenchmarkId::new("MergeSort", size),
            size,
            |b, &size| {
                b.iter(|| merge_sort(black_box(generate_vec(size))));
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

This creates a table named "Sorting Algorithms" with benchmarks for QuickSort and MergeSort at different sizes.

## Understanding the Output

### Table Columns

| Column | Description | Example |
|--------|-------------|---------|
| **Benchmark** | Name of the benchmark (from ID) | `Recursive/10` |
| **Mean** | Average time with CI | `1.24 μs [1.20 μs, 1.28 μs]` |
| **Median** | Median time with CI | `1.24 μs [1.22 μs, 1.26 μs]` |
| **MAD** | Median Absolute Deviation with CI | `25.00 ns [20.00 ns, 30.00 ns]` |
| **Throughput** | Elements/bytes per iteration | `1024 bytes/iter` |
| **Change** | % change from baseline | `🟢 +2.50% [-1.20%, +6.80%]` |

### Change Indicators

- 🟢 **Green**: Performance is similar or better (absolute change < 5% or negative)
- 🔴 **Red**: Performance regression detected (change > 5%)
- ⚪ **White**: No baseline for comparison (first run)

### Time Unit Conversion

The tool automatically converts nanoseconds to more readable units:

- **ns** (nanoseconds): < 1,000 ns
- **μs** (microseconds): 1,000 - 999,999 ns
- **ms** (milliseconds): 1,000,000 - 999,999,999 ns
- **s** (seconds): ≥ 1,000,000,000 ns

## Advanced Usage

### Running Specific Benchmarks

```bash
# Run only one benchmark file
cargo criterion --bench my_benchmark --message-format=json | criterion-for-human > BENCHMARKS.md

# Run benchmarks matching a pattern
cargo criterion --bench '*sort*' --message-format=json | criterion-for-human > BENCHMARKS.md
```

### Combining Multiple Benchmark Files

```bash
# Run benchmarks separately
cargo criterion --bench fibonacci --message-format=json > fib.json
cargo criterion --bench sorting --message-format=json > sort.json

# Combine results
cat fib.json sort.json | criterion-for-human > BENCHMARKS.md
```

### Setting Throughput

To show throughput in the output, configure it in your benchmark:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

pub fn benchmark_with_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("Data Processing");
    
    let data_size = 1024;
    group.throughput(Throughput::Bytes(data_size));
    
    group.bench_function("Process 1KB", |b| {
        let data = vec![0u8; data_size as usize];
        b.iter(|| process_data(black_box(&data)));
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_with_throughput);
criterion_main!(benches);
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Benchmark

on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install criterion-for-human
        run: cargo install --path .
      
      - name: Run benchmarks
        run: |
          cargo criterion --message-format=json | criterion-for-human > BENCHMARKS.md
      
      - name: Upload results
        uses: actions/upload-artifact@v2
        with:
          name: benchmark-results
          path: BENCHMARKS.md
```

## Troubleshooting

### No Output Generated

Make sure:
1. Your benchmarks are producing JSON output with `--message-format=json`
2. The JSON is being piped correctly
3. Benchmarks are actually running (check for errors)

### Missing Statistics

Some statistics may not be available:
- **Throughput**: Only shown if configured in benchmark
- **Change**: Only shown after a baseline is established (second run)
- **Slope/Typical**: Optional fields that may not always be present

### Benchmark IDs Not Grouping Correctly

Ensure your benchmark IDs use forward slashes (`/`) as separators:
- ✅ Good: `"MyGroup/BenchName/Param"`
- ❌ Bad: `"MyGroup-BenchName-Param"`

## Example Repository

See the `benches/example_benchmark.rs` file in this repository for a complete working example.

To run it:
```bash
cargo criterion --bench example_benchmark --message-format=json | cargo run > example_output.md
```
