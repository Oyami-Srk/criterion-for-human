use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Deserialize, Serialize)]
struct Estimate {
    estimate: f64,
    lower_bound: f64,
    upper_bound: f64,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChangeEstimate {
    estimate: f64,
    lower_bound: f64,
    upper_bound: f64,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Change {
    mean: ChangeEstimate,
    median: ChangeEstimate,
}

#[derive(Debug, Deserialize, Serialize)]
struct Throughput {
    per_iteration: u64,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

fn format_time(value: f64, unit: &str) -> String {
    if unit == "ns" {
        if value >= 1_000_000_000.0 {
            format!("{:.2} s", value / 1_000_000_000.0)
        } else if value >= 1_000_000.0 {
            format!("{:.2} ms", value / 1_000_000.0)
        } else if value >= 1_000.0 {
            format!("{:.2} μs", value / 1_000.0)
        } else {
            format!("{:.2} ns", value)
        }
    } else {
        format!("{:.2} {}", value, unit)
    }
}

fn format_estimate(est: &Estimate) -> String {
    format!(
        "{} [{}, {}]",
        format_time(est.estimate, &est.unit),
        format_time(est.lower_bound, &est.unit),
        format_time(est.upper_bound, &est.unit)
    )
}

fn format_change(change: &ChangeEstimate) -> String {
    let sign = if change.estimate >= 0.0 { "+" } else { "" };
    let color = if change.estimate.abs() < 5.0 {
        "🟢"
    } else if change.estimate > 0.0 {
        "🔴"
    } else {
        "🟢"
    };
    
    format!(
        "{} {}{:.2}% [{:+.2}%, {:+.2}%]",
        color,
        sign,
        change.estimate,
        change.lower_bound,
        change.upper_bound
    )
}

fn format_throughput(throughput: &[Throughput]) -> String {
    throughput
        .iter()
        .map(|t| format!("{} {}/iter", t.per_iteration, t.unit))
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_markdown_table(results: Vec<BenchmarkResult>) -> String {
    if results.is_empty() {
        return String::from("No benchmark results found.\n");
    }

    let mut output = String::new();
    
    // Title
    output.push_str("# 📊 Benchmark Results\n\n");
    output.push_str(&format!("**Total Benchmarks:** {}\n\n", results.len()));
    
    // Group results by table name (first part of ID before '/')
    let mut grouped: HashMap<String, Vec<&BenchmarkResult>> = HashMap::new();
    for result in &results {
        let parts: Vec<&str> = result.id.split('/').collect();
        let table_name = if parts.len() > 1 {
            parts[0].to_string()
        } else {
            "General".to_string()
        };
        grouped.entry(table_name).or_insert_with(Vec::new).push(result);
    }
    
    // Generate table for each group
    for (table_name, group_results) in grouped.iter() {
        output.push_str(&format!("## {}\n\n", table_name));
        
        // Table header
        output.push_str("| Benchmark | Mean | Median | MAD | Throughput | Change |\n");
        output.push_str("|-----------|------|--------|-----|------------|--------|\n");
        
        for result in group_results {
            let parts: Vec<&str> = result.id.split('/').collect();
            let bench_name = if parts.len() > 1 {
                parts[1..].join("/")
            } else {
                result.id.clone()
            };
            
            let mean_str = result.mean.as_ref()
                .map(|m| format_estimate(m))
                .unwrap_or_else(|| "N/A".to_string());
            
            let median_str = result.median.as_ref()
                .map(|m| format_estimate(m))
                .unwrap_or_else(|| "N/A".to_string());
            
            let mad_str = result.median_abs_dev.as_ref()
                .map(|m| format_estimate(m))
                .unwrap_or_else(|| "N/A".to_string());
            
            let throughput_str = result.throughput.as_ref()
                .map(|t| format_throughput(t))
                .unwrap_or_else(|| "N/A".to_string());
            
            let change_str = result.change.as_ref()
                .map(|c| format_change(&c.mean))
                .unwrap_or_else(|| "⚪ No baseline".to_string());
            
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                bench_name,
                mean_str,
                median_str,
                mad_str,
                throughput_str,
                change_str
            ));
        }
        
        output.push_str("\n");
    }
    
    // Add legend
    output.push_str("---\n\n");
    output.push_str("### 📖 Legend\n\n");
    output.push_str("- **Mean**: Average time per iteration with 95% confidence interval\n");
    output.push_str("- **Median**: Median time per iteration with 95% confidence interval\n");
    output.push_str("- **MAD**: Median Absolute Deviation with 95% confidence interval\n");
    output.push_str("- **Throughput**: Number of elements/bytes processed per iteration\n");
    output.push_str("- **Change**: Performance change compared to baseline\n");
    output.push_str("  - 🟢 Green: Improved or no significant change (<5%)\n");
    output.push_str("  - 🔴 Red: Regression (>5% slower)\n");
    output.push_str("  - ⚪ White: No baseline for comparison\n\n");
    
    output.push_str("*All values show estimate [lower bound, upper bound] with 95% confidence intervals*\n");
    
    output
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut results = Vec::new();
    
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line from stdin")?;
        
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        
        // Try to parse as JSON
        match serde_json::from_str::<BenchmarkResult>(&line) {
            Ok(result) => {
                // Only process benchmark-complete messages
                if result.reason == "benchmark-complete" {
                    results.push(result);
                }
            }
            Err(_) => {
                // Skip lines that aren't valid benchmark results
                continue;
            }
        }
    }
    
    let markdown = generate_markdown_table(results);
    println!("{}", markdown);
    
    Ok(())
}
