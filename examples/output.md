# 📊 Criterion Benchmark Report

<blockquote>

**6** benchmarks across **2** groups · **1** regression · **1** improvement · **2** without baseline

</blockquote>

> [!CAUTION]
> **Performance regressions detected:**
> - `String Operations/Concat/Large (1KB)` — +12.50% slower (mean)

> [!TIP]
> **Performance improvements:**
> - `Fibonacci/Iterative/10` — -8.50% faster (mean)

## 📦 `Fibonacci`

| | Benchmark | Time (mean) | vs fastest | Stability | Change |
|---|-----------|-------------|------------|-----------|--------|
| 🥇 | **Iterative/10** | `15.00 ns` | 🏆 **fastest** | ⚠️ | ✅ -8.50% |
| 🥈 | **Iterative/20** | `25.00 ns` | 1.67x slower | ⚠️ | ⚡ +1.20% |
| 🥉 | **Recursive/10** | `1.24 µs` | 82.80x slower | ⚠️ | 🆕 new |
|    | **Recursive/20** | `116.67 µs` | 7777.78x slower | 🥈 | 📊 +2.50% |

<details>
<summary>📊 Relative Performance</summary>

```
  Iterative/10  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 15.00 ns
  Iterative/20  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 25.00 ns
  Recursive/10  ▎░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1.24 µs
  Recursive/20  ██████████████████████████████ 116.67 µs
```

</details>

<details>
<summary>🔬 Detailed Statistics</summary>

### `Recursive/10`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `1.2420 µs` | 1.2000 µs | 1.2800 µs | ±40.00 ns |
| **Median** | `1.2400 µs` | 1.2200 µs | 1.2600 µs | ±20.00 ns |
| **MAD** | `25.0000 ns` | 20.0000 ns | 30.0000 ns | ±5.00 ns |

**Stability:** ⚠️ Fair — CV = 2.02%

---

### `Recursive/20`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `116.6667 µs` | 115.0000 µs | 118.5000 µs | ±1.75 µs |
| **Median** | `116.5000 µs` | 115.5000 µs | 117.5000 µs | ±1.00 µs |
| **MAD** | `1.0000 µs` | 800.0000 ns | 1.2000 µs | ±200.00 ns |

**Throughput:** 1 elements/iter

**Stability:** 🥈 Great — CV = 0.86%

**Change from baseline:** **📊 Slight Regression** (+2.50%)
  - Mean: `+2.50%` (95% CI: `-1.20%` to `+6.80%`)
  - Median: `+2.30%` (95% CI: `-0.50%` to `+5.10%`)
  - ℹ️ CI includes zero — change is **not** statistically significant

---

### `Iterative/10`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `15.0000 ns` | 14.5000 ns | 15.6000 ns | ±0.55 ns |
| **Median** | `14.9000 ns` | 14.7000 ns | 15.1000 ns | ±0.20 ns |
| **MAD** | `0.3000 ns` | 0.2000 ns | 0.4000 ns | ±0.10 ns |

**Stability:** ⚠️ Fair — CV = 2.01%

**Change from baseline:** **✅ Improved** (-8.50%)
  - Mean: `-8.50%` (95% CI: `-12.30%` to `-5.20%`)
  - Median: `-8.20%` (95% CI: `-11.50%` to `-4.90%`)
  - ✅ CI does not include zero — improvement is statistically significant

---

### `Iterative/20`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `25.0000 ns` | 24.0000 ns | 26.2000 ns | ±1.10 ns |
| **Median** | `24.8000 ns` | 24.5000 ns | 25.2000 ns | ±0.35 ns |
| **MAD** | `0.5000 ns` | 0.3000 ns | 0.7000 ns | ±0.20 ns |

**Throughput:** 1 elements/iter

**Stability:** ⚠️ Fair — CV = 2.02%

**Change from baseline:** **⚡ Unchanged** (+1.20%)
  - Mean: `+1.20%` (95% CI: `-2.10%` to `+4.50%`)
  - Median: `+0.80%` (95% CI: `-1.50%` to `+3.20%`)
  - ℹ️ CI includes zero — change is **not** statistically significant

---

</details>

## 📦 `String Operations`

| | Benchmark | Time (mean) | vs fastest | Stability | Change |
|---|-----------|-------------|------------|-----------|--------|
| 🥇 | **Concat/Small (10 bytes)** | `10.00 ns` | 🏆 **fastest** | 🥉 | 🆕 new |
| 🥈 | **Concat/Large (1KB)** | `1.50 µs` | 150.00x slower | 🥉 | ⚠️ +12.50% |

<details>
<summary>📊 Relative Performance</summary>

```
  Concat/Small (10 bytes)  ▏░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 10.00 ns
       Concat/Large (1KB)  ██████████████████████████████ 1.50 µs
```

</details>

<details>
<summary>🔬 Detailed Statistics</summary>

### `Concat/Small (10 bytes)`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `10.0000 ns` | 9.8000 ns | 10.4000 ns | ±0.30 ns |
| **Median** | `9.9500 ns` | 9.8500 ns | 10.0500 ns | ±0.10 ns |
| **MAD** | `0.1500 ns` | 0.1000 ns | 0.2000 ns | ±0.05 ns |

**Throughput:** 10 bytes/iter

**Stability:** 🥉 Good — CV = 1.51%

---

### `Concat/Large (1KB)`

| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |
|-----------|-------|-------------|-------------|----------|
| **Mean** | `1.5000 µs` | 1.4800 µs | 1.5300 µs | ±25.00 ns |
| **Median** | `1.4950 µs` | 1.4850 µs | 1.5100 µs | ±12.50 ns |
| **MAD** | `18.0000 ns` | 15.0000 ns | 22.0000 ns | ±3.50 ns |

**Throughput:** 1024 bytes/iter

**Stability:** 🥉 Good — CV = 1.20%

**Change from baseline:** **⚠️ Regression** (+12.50%)
  - Mean: `+12.50%` (95% CI: `+8.30%` to `+16.70%`)
  - Median: `+11.80%` (95% CI: `+7.90%` to `+15.60%`)
  - ⚠️ CI does not include zero — regression is statistically significant

---

</details>

## 🏅 Overall Leaderboard

| Rank | Benchmark | Mean | Relative | Stability |
|------|-----------|------|----------|----------|
| 🥇 | `String Operations/Concat/Small (10 bytes)` | `10.00 ns` | baseline | 🥉 (1.51%) |
| 🥈 | `Fibonacci/Iterative/10` | `15.00 ns` | 1.5x | ⚠️ (2.01%) |
| 🥉 | `Fibonacci/Iterative/20` | `25.00 ns` | 2.5x | ⚠️ (2.02%) |
| #4 | `Fibonacci/Recursive/10` | `1.24 µs` | 124.2x | ⚠️ (2.02%) |
| #5 | `String Operations/Concat/Large (1KB)` | `1.50 µs` | 150.0x | 🥉 (1.20%) |
| #6 | `Fibonacci/Recursive/20` | `116.67 µs` | 11666.7x | 🥈 (0.86%) |

<details>
<summary>📖 Legend & Methodology</summary>

#### Columns

| Column | Description |
|--------|-------------|
| **Time (mean)** | Average time per iteration |
| **vs fastest** | How much slower than the fastest in the group |
| **Stability** | Based on Coefficient of Variation (MAD/Median) |
| **Change** | Performance change vs. previous baseline run |

#### Stability Grades

| Grade | CV Range | Meaning |
|-------|----------|--------|
| 🥇 Excellent | < 0.5% | Near-zero variance |
| 🥈 Great | 0.5 – 1% | Very low variance |
| 🥉 Good | 1 – 2% | Acceptable variance |
| ⚠️ Fair | 2 – 5% | Noticeable variance |
| 🚨 Unstable | > 5% | High variance, results may be unreliable |

#### Change Indicators

| Icon | Meaning |
|------|--------|
| 🚀 | Major improvement (> 10% faster) |
| ✅ | Improvement (2 – 10% faster) |
| ⚡ | Unchanged (< 2% change) |
| 📊 | Slight regression (2 – 5% slower) |
| ⚠️ | Regression (5 – 15% slower) |
| 🔴 | Major regression (> 15% slower) |
| 🆕 | No baseline for comparison |

#### Methodology

- All confidence intervals are at the **95%** level
- **CV** (Coefficient of Variation) = MAD / Median × 100%
- Statistical significance is determined by whether the CI for change includes zero
- Relative performance ("vs fastest") compares mean times within each group

</details>
