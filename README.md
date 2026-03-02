# criterion-for-human

Render criterion JSON output in fancy yet detailed markdown output for mankind.

## 🎯 Features

- **📊 Executive Summary** — Instant overview: total benchmarks, regressions, improvements, and new benchmarks at a glance
- **🚨 Alert Boxes** — GitHub-flavored `[!CAUTION]` and `[!TIP]` callouts for regressions and improvements
- **🏆 Ranked Overview Tables** — Benchmarks sorted by speed with medals (🥇🥈🥉), relative speedup ratios, stability grades, and change badges
- **📊 Unicode Bar Charts** — Visual relative performance comparison using `█░` block characters
- **🔬 Detailed Statistics Cards** — Per-benchmark expandable sections with full mean/median/MAD/slope, CI widths, throughput, stability CV, and change analysis
- **📈 Statistical Significance** — Automatic detection of whether a change is statistically significant based on CI including zero
- **🏅 Cross-Group Leaderboard** — Overall ranking of all benchmarks across all groups
- **Stability Grades** — 🥇 Excellent / 🥈 Great / 🥉 Good / ⚠️ Fair / 🚨 Unstable based on Coefficient of Variation
- **Change Indicators** — 🚀 Major Improvement / ✅ Improved / ⚡ Unchanged / 📊 Slight Regression / ⚠️ Regression / 🔴 Major Regression / 🆕 New
- **Auto-Scaling Units** — Converts ns → µs → ms → s based on magnitude

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

```bash
# Single step — run benchmarks and generate report
cargo criterion --message-format=json | criterion-for-human > BENCHMARKS.md

# Or save JSON first, then generate report
cargo criterion --message-format=json > benchmarks.json
cat benchmarks.json | criterion-for-human > BENCHMARKS.md

# Combine multiple benchmark files
cat fib.json sort.json | criterion-for-human > BENCHMARKS.md
```

For a detailed usage guide, see [USAGE.md](USAGE.md).

## 📸 Example Output

Run with the included sample data:

```bash
cat examples/sample_output.json | cargo run --quiet
```

The tool produces a rich markdown report (see [examples/output.md](examples/output.md)):

### Executive Summary

> **6** benchmarks across **2** groups · **1** regression · **1** improvement · **2** without baseline

### Ranked Overview (per group)

| | Benchmark | Time (mean) | vs fastest | Stability | Change |
|---|-----------|-------------|------------|-----------|--------|
| 🥇 | **Iterative/10** | `15.00 ns` | 🏆 **fastest** | ⚠️ | ✅ -8.50% |
| 🥈 | **Iterative/20** | `25.00 ns` | 1.67x slower | ⚠️ | ⚡ +1.20% |
| 🥉 | **Recursive/10** | `1.24 µs` | 82.80x slower | ⚠️ | 🆕 new |

### Relative Performance Bar Chart

```
  Iterative/10  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 15.00 ns
  Iterative/20  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 25.00 ns
  Recursive/10  ▎░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1.24 µs
  Recursive/20  ██████████████████████████████ 116.67 µs
```

### Detailed Statistics (expandable)

Each benchmark gets a full statistics table with CI widths, stability grade, throughput, and statistical significance analysis of changes.

### Cross-Group Leaderboard

All benchmarks ranked globally by speed with relative performance ratios.

## 📊 Benchmark ID Format

Structure your benchmark IDs as:

```
<group_name>/<benchmark_name>[/<parameter>]
```

The first segment becomes the group heading. Everything after is the benchmark name.

## 🆚 Comparison with criterion-table

| Feature | criterion-for-human | criterion-table |
|---------|-------------------|----------------|
| Executive summary | ✅ | ❌ |
| Regression/improvement alerts | ✅ (GH callouts) | ❌ |
| Ranked overview with medals | ✅ | ❌ |
| Relative speedup ratios | ✅ | ❌ |
| Unicode bar charts | ✅ | ❌ |
| Stability grades (CV) | ✅ | ❌ |
| Statistical significance | ✅ | ❌ |
| Per-benchmark detail cards | ✅ | ❌ |
| CI width display | ✅ | ❌ |
| Cross-group leaderboard | ✅ | ❌ |
| Throughput | ✅ | ✅ |
| Change % | ✅ (6 tiers) | ✅ (basic) |
| Auto time units | ✅ | ✅ |
| Confidence intervals | ✅ (all metrics) | ❌ |

## 🔧 Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with example data
cat examples/sample_output.json | cargo run --quiet
```

## 📝 License

MIT or Apache-2.0 (your choice)

