# 📊 Benchmark Results

**Total Benchmarks:** 6

## String Operations

| Benchmark | Mean | Median | MAD | Throughput | Change |
|-----------|------|--------|-----|------------|--------|
| Concat/Small (10 bytes) | 10.00 ns [9.80 ns, 10.40 ns] | 9.95 ns [9.85 ns, 10.05 ns] | 0.15 ns [0.10 ns, 0.20 ns] | 10 bytes/iter | ⚪ No baseline |
| Concat/Large (1KB) | 1.50 μs [1.48 μs, 1.53 μs] | 1.50 μs [1.49 μs, 1.51 μs] | 18.00 ns [15.00 ns, 22.00 ns] | 1024 bytes/iter | 🔴 +12.50% [+8.30%, +16.70%] |

## Fibonacci

| Benchmark | Mean | Median | MAD | Throughput | Change |
|-----------|------|--------|-----|------------|--------|
| Recursive/10 | 1.24 μs [1.20 μs, 1.28 μs] | 1.24 μs [1.22 μs, 1.26 μs] | 25.00 ns [20.00 ns, 30.00 ns] | N/A | ⚪ No baseline |
| Recursive/20 | 116.67 μs [115.00 μs, 118.50 μs] | 116.50 μs [115.50 μs, 117.50 μs] | 1.00 μs [800.00 ns, 1.20 μs] | 1 elements/iter | 🟢 +2.50% [-1.20%, +6.80%] |
| Iterative/10 | 15.00 ns [14.50 ns, 15.60 ns] | 14.90 ns [14.70 ns, 15.10 ns] | 0.30 ns [0.20 ns, 0.40 ns] | N/A | 🟢 -8.50% [-12.30%, -5.20%] |
| Iterative/20 | 25.00 ns [24.00 ns, 26.20 ns] | 24.80 ns [24.50 ns, 25.20 ns] | 0.50 ns [0.30 ns, 0.70 ns] | 1 elements/iter | 🟢 +1.20% [-2.10%, +4.50%] |

---

### 📖 Legend

- **Mean**: Average time per iteration with 95% confidence interval
- **Median**: Median time per iteration with 95% confidence interval
- **MAD**: Median Absolute Deviation with 95% confidence interval
- **Throughput**: Number of elements/bytes processed per iteration
- **Change**: Performance change compared to baseline
  - 🟢 Green: Improved or no significant change (<5%)
  - 🔴 Red: Regression (>5% slower)
  - ⚪ White: No baseline for comparison

*All values show estimate [lower bound, upper bound] with 95% confidence intervals*

