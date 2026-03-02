# criterion-for-human

Render criterion JSON output in fancy yet detailed markdown output for mankind

## 🎯 Features

- **Detailed Statistics**: Shows mean, median, MAD (Median Absolute Deviation), and confidence intervals
- **Throughput Metrics**: Displays throughput information when available
- **Performance Comparison**: Compares against baseline with visual indicators
- **Human-Readable Format**: Automatic time unit conversion (ns → μs → ms → s)
- **Visual Indicators**: Color-coded emoji indicators for performance changes
  - 🟢 Green: Improved or no significant change (<5%)
  - 🔴 Red: Regression (>5% slower)
  - ⚪ White: No baseline for comparison
- **Grouped Tables**: Automatically groups benchmarks by category

## 📦 Installation

```bash
cargo install --path .
```

Or build from source:

```bash
git clone https://github.com/Oyami-Srk/criterion-for-human.git
cd criterion-for-human
cargo build --release
```

## 🚀 Usage

### Basic Usage

The tool reads criterion's JSON output from stdin and outputs markdown to stdout:

```bash
# Run benchmarks and pipe directly to criterion-for-human
cargo criterion --message-format=json | criterion-for-human > BENCHMARKS.md

# Or save the JSON first and process it later
cargo criterion --message-format=json > benchmarks.json
cat benchmarks.json | criterion-for-human > BENCHMARKS.md
```

### Example Output

See [examples/sample_output.json](examples/sample_output.json) for sample input data.

Run the example:

```bash
cat examples/sample_output.json | cargo run > examples/output.md
```

This will generate a markdown table like:

```markdown
# 📊 Benchmark Results

**Total Benchmarks:** 6

## Fibonacci

| Benchmark | Mean | Median | MAD | Throughput | Change |
|-----------|------|--------|-----|------------|--------|
| Recursive/10 | 1.24 μs [1.20 μs, 1.28 μs] | 1.24 μs [1.22 μs, 1.26 μs] | 25.00 ns [20.00 ns, 30.00 ns] | N/A | ⚪ No baseline |
| Recursive/20 | 116.67 μs [115.00 μs, 118.50 μs] | 116.50 μs [115.50 μs, 117.50 μs] | 1.00 μs [800.00 ns, 1.20 μs] | 1 elements/iter | 🟢 +2.50% [-1.20%, +6.80%] |
...
```

## 📊 Benchmark ID Format

For best results, structure your benchmark IDs as follows:

```
<table_name>/<column_name>/<row_name>
```

Example:
```rust
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    
    for size in [10, 20].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", size), size, |b, &size| {
            b.iter(|| fibonacci(black_box(size)));
        });
        
        group.bench_with_input(BenchmarkId::new("Iterative", size), size, |b, &size| {
            b.iter(|| fibonacci_iterative(black_box(size)));
        });
    }
    
    group.finish();
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

## 🆚 Comparison with criterion-table

This tool provides more detailed information compared to [criterion-table](https://github.com/nu11ptr/criterion-table/):

| Feature | criterion-for-human | criterion-table |
|---------|-------------------|----------------|
| Mean with CI | ✅ | ❌ |
| Median with CI | ✅ | ❌ |
| MAD with CI | ✅ | ❌ |
| Throughput | ✅ | ✅ |
| Change % with CI | ✅ | ✅ |
| Visual indicators | ✅ (emoji) | ❌ |
| Auto time units | ✅ | ✅ |
| Confidence intervals | ✅ (all metrics) | ❌ |

## 📖 Output Sections

### Statistics Columns

- **Mean**: Average time per iteration with 95% confidence interval
- **Median**: Median time per iteration with 95% confidence interval  
- **MAD**: Median Absolute Deviation with 95% confidence interval
- **Throughput**: Number of elements/bytes processed per iteration
- **Change**: Performance change compared to baseline

All values show: `estimate [lower bound, upper bound]` with 95% confidence intervals

## 🔧 Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with example data
cat examples/sample_output.json | cargo run
```

## 📝 License

MIT or Apache-2.0 (your choice)

