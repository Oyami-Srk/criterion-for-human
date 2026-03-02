use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

// ── Data Model ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Estimate {
    estimate: f64,
    lower_bound: f64,
    upper_bound: f64,
    unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChangeEstimate {
    estimate: f64,
    lower_bound: f64,
    upper_bound: f64,
    unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Change {
    mean: ChangeEstimate,
    median: ChangeEstimate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Throughput {
    per_iteration: u64,
    unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BenchmarkResult {
    reason: String,
    id: String,
    #[serde(default)]
    report_directory: String,
    #[serde(default)]
    typical: Option<Estimate>,
    #[serde(default)]
    mean: Option<Estimate>,
    #[serde(default)]
    median: Option<Estimate>,
    #[serde(default)]
    median_abs_dev: Option<Estimate>,
    #[serde(default)]
    slope: Option<Estimate>,
    #[serde(default)]
    change: Option<Change>,
    #[serde(default)]
    throughput: Option<Vec<Throughput>>,
}

// ── Formatting Helpers ──────────────────────────────────────────────

fn format_time(value: f64, unit: &str) -> String {
    if unit == "ns" {
        if value >= 1_000_000_000.0 {
            format!("{:.4} s", value / 1_000_000_000.0)
        } else if value >= 1_000_000.0 {
            format!("{:.4} ms", value / 1_000_000.0)
        } else if value >= 1_000.0 {
            format!("{:.4} µs", value / 1_000.0)
        } else {
            format!("{:.4} ns", value)
        }
    } else {
        format!("{:.4} {}", value, unit)
    }
}

fn format_time_short(value: f64, unit: &str) -> String {
    if unit == "ns" {
        if value >= 1_000_000_000.0 {
            format!("{:.2} s", value / 1_000_000_000.0)
        } else if value >= 1_000_000.0 {
            format!("{:.2} ms", value / 1_000_000.0)
        } else if value >= 1_000.0 {
            format!("{:.2} µs", value / 1_000.0)
        } else {
            format!("{:.2} ns", value)
        }
    } else {
        format!("{:.2} {}", value, unit)
    }
}

/// Returns the mean estimate in nanoseconds (assumes unit is "ns")
fn mean_ns(result: &BenchmarkResult) -> f64 {
    result
        .mean
        .as_ref()
        .map(|m| m.estimate)
        .unwrap_or(f64::MAX)
}

/// Coefficient of variation: MAD / median (as percentage)
fn coeff_of_variation(result: &BenchmarkResult) -> Option<f64> {
    let mad = result.median_abs_dev.as_ref()?.estimate;
    let median = result.median.as_ref()?.estimate;
    if median == 0.0 {
        return None;
    }
    Some((mad / median) * 100.0)
}

/// Performance grade based on coefficient of variation
fn stability_grade(cv: f64) -> &'static str {
    if cv < 0.5 {
        "🥇 Excellent"
    } else if cv < 1.0 {
        "🥈 Great"
    } else if cv < 2.0 {
        "🥉 Good"
    } else if cv < 5.0 {
        "⚠️ Fair"
    } else {
        "🚨 Unstable"
    }
}

fn stability_grade_short(cv: f64) -> &'static str {
    if cv < 0.5 {
        "🥇"
    } else if cv < 1.0 {
        "🥈"
    } else if cv < 2.0 {
        "🥉"
    } else if cv < 5.0 {
        "⚠️"
    } else {
        "🚨"
    }
}

/// Build a Unicode bar (filled portion of a fixed-width bar)
fn unicode_bar(fraction: f64, width: usize) -> String {
    let blocks = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
    let total_eighths = (fraction.clamp(0.0, 1.0) * (width as f64) * 8.0) as usize;
    let full = total_eighths / 8;
    let remainder = total_eighths % 8;

    let mut bar = "█".repeat(full);
    if remainder > 0 && full < width {
        bar.push_str(blocks[remainder]);
    }
    // Pad with light shade to the width
    let current_visual_len = if remainder > 0 { full + 1 } else { full };
    for _ in current_visual_len..width {
        bar.push('░');
    }
    bar
}

fn change_badge(change: &ChangeEstimate) -> String {
    let pct = change.estimate;
    if pct.abs() < 2.0 {
        format!("**⚡ Unchanged** ({:+.2}%)", pct)
    } else if pct < -10.0 {
        format!("**🚀 Major Improvement** ({:+.2}%)", pct)
    } else if pct < -2.0 {
        format!("**✅ Improved** ({:+.2}%)", pct)
    } else if pct < 5.0 {
        format!("**📊 Slight Regression** ({:+.2}%)", pct)
    } else if pct < 15.0 {
        format!("**⚠️ Regression** ({:+.2}%)", pct)
    } else {
        format!("**🔴 Major Regression** ({:+.2}%)", pct)
    }
}

fn change_badge_short(change: &ChangeEstimate) -> String {
    let pct = change.estimate;
    if pct.abs() < 2.0 {
        format!("⚡ {:+.2}%", pct)
    } else if pct < -10.0 {
        format!("🚀 {:+.2}%", pct)
    } else if pct < -2.0 {
        format!("✅ {:+.2}%", pct)
    } else if pct < 5.0 {
        format!("📊 {:+.2}%", pct)
    } else if pct < 15.0 {
        format!("⚠️ {:+.2}%", pct)
    } else {
        format!("🔴 {:+.2}%", pct)
    }
}

fn format_throughput(throughput: &[Throughput]) -> String {
    throughput
        .iter()
        .map(|t| format!("{} {}/iter", t.per_iteration, t.unit))
        .collect::<Vec<_>>()
        .join(", ")
}

fn bench_name(result: &BenchmarkResult) -> String {
    let parts: Vec<&str> = result.id.split('/').collect();
    if parts.len() > 1 {
        parts[1..].join("/")
    } else {
        result.id.clone()
    }
}

fn group_name(result: &BenchmarkResult) -> String {
    let parts: Vec<&str> = result.id.split('/').collect();
    if parts.len() > 1 {
        parts[0].to_string()
    } else {
        "General".to_string()
    }
}

// ── Report Generation ───────────────────────────────────────────────

fn generate_report(results: Vec<BenchmarkResult>) -> String {
    if results.is_empty() {
        return String::from("No benchmark results found.\n");
    }

    let mut output = String::new();

    // Group results preserving insertion order
    let mut group_order: Vec<String> = Vec::new();
    let mut grouped: Vec<(String, Vec<&BenchmarkResult>)> = Vec::new();
    for result in &results {
        let gn = group_name(result);
        if let Some(pos) = group_order.iter().position(|g| g == &gn) {
            grouped[pos].1.push(result);
        } else {
            group_order.push(gn.clone());
            grouped.push((gn, vec![result]));
        }
    }

    // ── Executive Summary ───────────────────────────────────────────
    output.push_str("# 📊 Criterion Benchmark Report\n\n");

    // Compute top-level stats
    let total = results.len();
    let regressions: Vec<&BenchmarkResult> = results
        .iter()
        .filter(|r| {
            r.change
                .as_ref()
                .map(|c| c.mean.estimate > 5.0)
                .unwrap_or(false)
        })
        .collect();
    let improvements: Vec<&BenchmarkResult> = results
        .iter()
        .filter(|r| {
            r.change
                .as_ref()
                .map(|c| c.mean.estimate < -2.0)
                .unwrap_or(false)
        })
        .collect();
    let no_baseline: usize = results.iter().filter(|r| r.change.is_none()).count();

    output.push_str("<blockquote>\n\n");
    output.push_str(&format!(
        "**{}** benchmarks across **{}** groups",
        total,
        grouped.len()
    ));
    if !regressions.is_empty() {
        output.push_str(&format!(
            " · **{}** regression{}",
            regressions.len(),
            if regressions.len() == 1 { "" } else { "s" }
        ));
    }
    if !improvements.is_empty() {
        output.push_str(&format!(
            " · **{}** improvement{}",
            improvements.len(),
            if improvements.len() == 1 { "" } else { "s" }
        ));
    }
    if no_baseline > 0 {
        output.push_str(&format!(
            " · **{}** without baseline",
            no_baseline
        ));
    }
    output.push_str("\n\n</blockquote>\n\n");

    // Quick alert boxes
    if !regressions.is_empty() {
        output.push_str("> [!CAUTION]\n");
        output.push_str("> **Performance regressions detected:**\n");
        for r in &regressions {
            if let Some(c) = &r.change {
                output.push_str(&format!(
                    "> - `{}` — {:+.2}% slower (mean)\n",
                    r.id, c.mean.estimate
                ));
            }
        }
        output.push('\n');
    }
    if !improvements.is_empty() {
        output.push_str("> [!TIP]\n");
        output.push_str("> **Performance improvements:**\n");
        for r in &improvements {
            if let Some(c) = &r.change {
                output.push_str(&format!(
                    "> - `{}` — {:+.2}% faster (mean)\n",
                    r.id, c.mean.estimate
                ));
            }
        }
        output.push('\n');
    }

    // ── Per-Group Sections ──────────────────────────────────────────
    for (gname, group) in &grouped {
        output.push_str(&format!("## {} `{}`\n\n", "📦", gname));

        // Sort by mean for ranking
        let mut sorted: Vec<&&BenchmarkResult> = group.iter().collect();
        sorted.sort_by(|a, b| mean_ns(a).partial_cmp(&mean_ns(b)).unwrap());

        let fastest_ns = mean_ns(sorted[0]);

        // ── Overview Table ──────────────────────────────────────────
        output.push_str("| | Benchmark | Time (mean) | vs fastest | Stability | Change |\n");
        output.push_str("|---|-----------|-------------|------------|-----------|--------|\n");

        for (rank, result) in sorted.iter().enumerate() {
            let name = bench_name(result);
            let m_ns = mean_ns(result);
            let mean_str = result
                .mean
                .as_ref()
                .map(|m| format_time_short(m.estimate, &m.unit))
                .unwrap_or_else(|| "N/A".into());

            let ratio = if fastest_ns > 0.0 {
                m_ns / fastest_ns
            } else {
                1.0
            };
            let ratio_str = if rank == 0 {
                "🏆 **fastest**".to_string()
            } else {
                format!("{:.2}x slower", ratio)
            };

            let cv = coeff_of_variation(result);
            let stability = cv
                .map(|v| stability_grade_short(v).to_string())
                .unwrap_or_else(|| "—".into());

            let change_str = result
                .change
                .as_ref()
                .map(|c| change_badge_short(&c.mean))
                .unwrap_or_else(|| "🆕 new".into());

            let rank_medal = match rank {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };

            output.push_str(&format!(
                "| {} | **{}** | `{}` | {} | {} | {} |\n",
                rank_medal, name, mean_str, ratio_str, stability, change_str
            ));
        }
        output.push('\n');

        // ── Relative Performance Bar Chart ──────────────────────────
        output.push_str("<details>\n<summary>📊 Relative Performance</summary>\n\n");
        output.push_str("```\n");
        // Find longest name for alignment
        let max_name_len = sorted
            .iter()
            .map(|r| bench_name(r).len())
            .max()
            .unwrap_or(10);
        let bar_width = 30;

        let slowest_ns = mean_ns(sorted.last().unwrap());
        for result in &sorted {
            let name = bench_name(result);
            let m_ns = mean_ns(result);
            let fraction = if slowest_ns > 0.0 {
                m_ns / slowest_ns
            } else {
                0.0
            };
            let mean_str = result
                .mean
                .as_ref()
                .map(|m| format_time_short(m.estimate, &m.unit))
                .unwrap_or_default();
            output.push_str(&format!(
                "  {:>width$}  {} {}\n",
                name,
                unicode_bar(fraction, bar_width),
                mean_str,
                width = max_name_len
            ));
        }
        output.push_str("```\n\n");
        output.push_str("</details>\n\n");

        // ── Detailed Benchmark Cards ────────────────────────────────
        output.push_str("<details>\n<summary>🔬 Detailed Statistics</summary>\n\n");

        for result in group {
            let name = bench_name(result);
            output.push_str(&format!("### `{}`\n\n", name));

            // Statistics table
            output.push_str("| Statistic | Value | 95% CI Lower | 95% CI Upper | CI Width |\n");
            output.push_str("|-----------|-------|-------------|-------------|----------|\n");

            if let Some(mean) = &result.mean {
                let ci_width = mean.upper_bound - mean.lower_bound;
                output.push_str(&format!(
                    "| **Mean** | `{}` | {} | {} | ±{} |\n",
                    format_time(mean.estimate, &mean.unit),
                    format_time(mean.lower_bound, &mean.unit),
                    format_time(mean.upper_bound, &mean.unit),
                    format_time_short(ci_width / 2.0, &mean.unit),
                ));
            }
            if let Some(median) = &result.median {
                let ci_width = median.upper_bound - median.lower_bound;
                output.push_str(&format!(
                    "| **Median** | `{}` | {} | {} | ±{} |\n",
                    format_time(median.estimate, &median.unit),
                    format_time(median.lower_bound, &median.unit),
                    format_time(median.upper_bound, &median.unit),
                    format_time_short(ci_width / 2.0, &median.unit),
                ));
            }
            if let Some(mad) = &result.median_abs_dev {
                let ci_width = mad.upper_bound - mad.lower_bound;
                output.push_str(&format!(
                    "| **MAD** | `{}` | {} | {} | ±{} |\n",
                    format_time(mad.estimate, &mad.unit),
                    format_time(mad.lower_bound, &mad.unit),
                    format_time(mad.upper_bound, &mad.unit),
                    format_time_short(ci_width / 2.0, &mad.unit),
                ));
            }
            if let Some(slope) = &result.slope {
                let ci_width = slope.upper_bound - slope.lower_bound;
                output.push_str(&format!(
                    "| **Slope** | `{}` | {} | {} | ±{} |\n",
                    format_time(slope.estimate, &slope.unit),
                    format_time(slope.lower_bound, &slope.unit),
                    format_time(slope.upper_bound, &slope.unit),
                    format_time_short(ci_width / 2.0, &slope.unit),
                ));
            }

            output.push('\n');

            // Throughput
            if let Some(tp) = &result.throughput {
                output.push_str(&format!("**Throughput:** {}\n\n", format_throughput(tp)));
            }

            // Stability assessment
            if let Some(cv) = coeff_of_variation(result) {
                output.push_str(&format!(
                    "**Stability:** {} — CV = {:.2}%\n\n",
                    stability_grade(cv),
                    cv
                ));
            }

            // Change analysis
            if let Some(change) = &result.change {
                output.push_str(&format!(
                    "**Change from baseline:** {}\n",
                    change_badge(&change.mean)
                ));
                output.push_str(&format!(
                    "  - Mean: `{:+.2}%` (95% CI: `{:+.2}%` to `{:+.2}%`)\n",
                    change.mean.estimate, change.mean.lower_bound, change.mean.upper_bound
                ));
                output.push_str(&format!(
                    "  - Median: `{:+.2}%` (95% CI: `{:+.2}%` to `{:+.2}%`)\n",
                    change.median.estimate,
                    change.median.lower_bound,
                    change.median.upper_bound
                ));

                // Statistical significance hint
                if change.mean.lower_bound > 0.0 {
                    output.push_str(
                        "  - ⚠️ CI does not include zero — regression is statistically significant\n",
                    );
                } else if change.mean.upper_bound < 0.0 {
                    output.push_str(
                        "  - ✅ CI does not include zero — improvement is statistically significant\n",
                    );
                } else {
                    output.push_str(
                        "  - ℹ️ CI includes zero — change is **not** statistically significant\n",
                    );
                }
                output.push('\n');
            }

            output.push_str("---\n\n");
        }

        output.push_str("</details>\n\n");
    }

    // ── Cross-Group Leaderboard ─────────────────────────────────────
    if grouped.len() > 1 || results.len() > 2 {
        output.push_str("## 🏅 Overall Leaderboard\n\n");

        let mut all_sorted: Vec<&BenchmarkResult> = results.iter().collect();
        all_sorted.sort_by(|a, b| mean_ns(a).partial_cmp(&mean_ns(b)).unwrap());

        let top_n = all_sorted.len().min(10);
        let fastest_overall = mean_ns(all_sorted[0]);

        output.push_str("| Rank | Benchmark | Mean | Relative | Stability |\n");
        output.push_str("|------|-----------|------|----------|----------|\n");

        for (i, result) in all_sorted.iter().take(top_n).enumerate() {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => &format!("#{}", i + 1),
            };
            let mean_str = result
                .mean
                .as_ref()
                .map(|m| format_time_short(m.estimate, &m.unit))
                .unwrap_or_else(|| "N/A".into());
            let ratio = mean_ns(result) / fastest_overall;
            let ratio_str = if i == 0 {
                "baseline".to_string()
            } else {
                format!("{:.1}x", ratio)
            };
            let cv = coeff_of_variation(result);
            let stability = cv
                .map(|v| format!("{} ({:.2}%)", stability_grade_short(v), v))
                .unwrap_or_else(|| "—".into());

            output.push_str(&format!(
                "| {} | `{}` | `{}` | {} | {} |\n",
                medal, result.id, mean_str, ratio_str, stability
            ));
        }
        output.push('\n');
    }

    // ── Legend ───────────────────────────────────────────────────────
    output.push_str("<details>\n<summary>📖 Legend & Methodology</summary>\n\n");
    output.push_str("#### Columns\n\n");
    output.push_str("| Column | Description |\n");
    output.push_str("|--------|-------------|\n");
    output.push_str("| **Time (mean)** | Average time per iteration |\n");
    output.push_str("| **vs fastest** | How much slower than the fastest in the group |\n");
    output.push_str("| **Stability** | Based on Coefficient of Variation (MAD/Median) |\n");
    output.push_str("| **Change** | Performance change vs. previous baseline run |\n\n");
    output.push_str("#### Stability Grades\n\n");
    output.push_str("| Grade | CV Range | Meaning |\n");
    output.push_str("|-------|----------|--------|\n");
    output.push_str("| 🥇 Excellent | < 0.5% | Near-zero variance |\n");
    output.push_str("| 🥈 Great | 0.5 – 1% | Very low variance |\n");
    output.push_str("| 🥉 Good | 1 – 2% | Acceptable variance |\n");
    output.push_str("| ⚠️ Fair | 2 – 5% | Noticeable variance |\n");
    output.push_str("| 🚨 Unstable | > 5% | High variance, results may be unreliable |\n\n");
    output.push_str("#### Change Indicators\n\n");
    output.push_str("| Icon | Meaning |\n");
    output.push_str("|------|--------|\n");
    output.push_str("| 🚀 | Major improvement (> 10% faster) |\n");
    output.push_str("| ✅ | Improvement (2 – 10% faster) |\n");
    output.push_str("| ⚡ | Unchanged (< 2% change) |\n");
    output.push_str("| 📊 | Slight regression (2 – 5% slower) |\n");
    output.push_str("| ⚠️ | Regression (5 – 15% slower) |\n");
    output.push_str("| 🔴 | Major regression (> 15% slower) |\n");
    output.push_str("| 🆕 | No baseline for comparison |\n\n");
    output.push_str("#### Methodology\n\n");
    output.push_str("- All confidence intervals are at the **95%** level\n");
    output.push_str("- **CV** (Coefficient of Variation) = MAD / Median × 100%\n");
    output.push_str("- Statistical significance is determined by whether the CI for change includes zero\n");
    output.push_str("- Relative performance (\"vs fastest\") compares mean times within each group\n\n");
    output.push_str("</details>\n");

    output
}

// ── Entry Point ─────────────────────────────────────────────────────

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut results = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line from stdin")?;

        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<BenchmarkResult>(&line) {
            Ok(result) => {
                if result.reason == "benchmark-complete" {
                    results.push(result);
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    let markdown = generate_report(results);
    print!("{}", markdown);

    Ok(())
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_result(id: &str, mean: f64, median: f64, mad: f64) -> BenchmarkResult {
        BenchmarkResult {
            reason: "benchmark-complete".into(),
            id: id.into(),
            report_directory: String::new(),
            typical: None,
            mean: Some(Estimate {
                estimate: mean,
                lower_bound: mean * 0.95,
                upper_bound: mean * 1.05,
                unit: "ns".into(),
            }),
            median: Some(Estimate {
                estimate: median,
                lower_bound: median * 0.95,
                upper_bound: median * 1.05,
                unit: "ns".into(),
            }),
            median_abs_dev: Some(Estimate {
                estimate: mad,
                lower_bound: mad * 0.8,
                upper_bound: mad * 1.2,
                unit: "ns".into(),
            }),
            slope: None,
            change: None,
            throughput: None,
        }
    }

    fn sample_result_with_change(
        id: &str,
        mean: f64,
        median: f64,
        mad: f64,
        change_pct: f64,
    ) -> BenchmarkResult {
        let mut r = sample_result(id, mean, median, mad);
        r.change = Some(Change {
            mean: ChangeEstimate {
                estimate: change_pct,
                lower_bound: change_pct - 3.0,
                upper_bound: change_pct + 3.0,
                unit: "%".into(),
            },
            median: ChangeEstimate {
                estimate: change_pct * 0.9,
                lower_bound: change_pct - 2.5,
                upper_bound: change_pct + 2.5,
                unit: "%".into(),
            },
        });
        r
    }

    #[test]
    fn empty_results() {
        let output = generate_report(vec![]);
        assert_eq!(output, "No benchmark results found.\n");
    }

    #[test]
    fn report_contains_title() {
        let results = vec![sample_result("Group/Bench1", 100.0, 99.0, 2.0)];
        let output = generate_report(results);
        assert!(output.contains("# 📊 Criterion Benchmark Report"));
    }

    #[test]
    fn report_groups_by_id() {
        let results = vec![
            sample_result("Alpha/Fast", 10.0, 10.0, 0.1),
            sample_result("Alpha/Slow", 1000.0, 1000.0, 5.0),
            sample_result("Beta/Only", 50.0, 50.0, 1.0),
        ];
        let output = generate_report(results);
        assert!(output.contains("📦 `Alpha`"));
        assert!(output.contains("📦 `Beta`"));
    }

    #[test]
    fn report_ranks_within_group() {
        let results = vec![
            sample_result("G/Slow", 1000.0, 1000.0, 5.0),
            sample_result("G/Fast", 10.0, 10.0, 0.1),
        ];
        let output = generate_report(results);
        assert!(output.contains("🏆 **fastest**"));
        assert!(output.contains("slower"));
    }

    #[test]
    fn report_shows_regression() {
        let results = vec![sample_result_with_change("G/Bad", 100.0, 100.0, 2.0, 12.0)];
        let output = generate_report(results);
        assert!(output.contains("regression"));
    }

    #[test]
    fn report_shows_improvement() {
        let results = vec![sample_result_with_change("G/Good", 100.0, 100.0, 2.0, -8.0)];
        let output = generate_report(results);
        assert!(output.contains("improvement"));
    }

    #[test]
    fn report_has_bar_chart() {
        let results = vec![
            sample_result("G/A", 100.0, 100.0, 1.0),
            sample_result("G/B", 200.0, 200.0, 2.0),
        ];
        let output = generate_report(results);
        assert!(output.contains("█"));
        assert!(output.contains("░"));
    }

    #[test]
    fn report_has_stability_grades() {
        let results = vec![
            sample_result("G/Stable", 100.0, 100.0, 0.1),
            sample_result("G/Unstable", 100.0, 100.0, 10.0),
        ];
        let output = generate_report(results);
        assert!(output.contains("🥇"));
    }

    #[test]
    fn report_has_leaderboard() {
        let results = vec![
            sample_result("A/X", 100.0, 100.0, 1.0),
            sample_result("B/Y", 200.0, 200.0, 2.0),
            sample_result("C/Z", 50.0, 50.0, 0.5),
        ];
        let output = generate_report(results);
        assert!(output.contains("🏅 Overall Leaderboard"));
    }

    #[test]
    fn report_statistical_significance() {
        // CI does not include zero (regression)
        let mut r = sample_result("G/Bench", 100.0, 100.0, 2.0);
        r.change = Some(Change {
            mean: ChangeEstimate {
                estimate: 10.0,
                lower_bound: 5.0,
                upper_bound: 15.0,
                unit: "%".into(),
            },
            median: ChangeEstimate {
                estimate: 9.0,
                lower_bound: 4.0,
                upper_bound: 14.0,
                unit: "%".into(),
            },
        });
        let output = generate_report(vec![r]);
        assert!(output.contains("statistically significant"));
    }

    #[test]
    fn format_time_scales_correctly() {
        assert_eq!(format_time(500.0, "ns"), "500.0000 ns");
        assert_eq!(format_time(1500.0, "ns"), "1.5000 µs");
        assert_eq!(format_time(1_500_000.0, "ns"), "1.5000 ms");
        assert_eq!(format_time(1_500_000_000.0, "ns"), "1.5000 s");
    }

    #[test]
    fn unicode_bar_extremes() {
        let empty = unicode_bar(0.0, 10);
        assert!(empty.contains("░"));
        let full = unicode_bar(1.0, 10);
        assert!(full.contains("█"));
        assert!(!full.contains("░"));
    }

    #[test]
    fn coeff_of_variation_calculation() {
        let r = sample_result("G/B", 100.0, 100.0, 2.0);
        let cv = coeff_of_variation(&r).unwrap();
        assert!((cv - 2.0).abs() < 0.01);
    }
}
