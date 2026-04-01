mod engine;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use colored::*;
use engine::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;

#[derive(Parser)]
#[command(
    name = "safe-vault",
    about = "🏦 Safe-Vault — Precision Financial Logic Engine\n   Zero rounding errors. Leap-year aware. Built in Rust."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prove that f64 floats drift — Decimal does not
    Proof {
        #[arg(short, long, default_value = "1000")]
        ops: u32,
    },
    /// Compound interest with true leap-year calendar accrual
    Interest {
        #[arg(short, long)]
        principal: String,
        #[arg(short, long)]
        rate: String,
        #[arg(long)]
        start: String,
        #[arg(long)]
        end: String,
        #[arg(long, default_value = "false")]
        compare: bool,
    },
    /// Run full demo with sample data
    Demo,
}

fn main() {
    print_banner();
    let cli = Cli::parse();

    match cli.command {
        Commands::Proof { ops } => run_proof(ops),
        Commands::Interest { principal, rate, start, end, compare } => {
            let p = parse_decimal(&principal, "principal");
            let r = parse_decimal(&rate, "rate");
            let s = parse_date(&start, "start");
            let e = parse_date(&end, "end");
            run_interest(p, r, s, e, compare);
        }
        Commands::Demo => run_demo(),
    }
}

fn run_proof(ops: u32) {
    println!("{}", "─".repeat(60).bright_red());
    println!("{}", "  🔬  FLOAT vs DECIMAL — PROOF OF ERROR".bold().white());
    println!("{}", "─".repeat(60).bright_red());

    let proof = prove_float_error(ops);

    println!("  Operation: Add {} × 0.1", ops.to_string().yellow().bold());
    println!("  Expected:  {}", format!("{}", ops / 10).green().bold());
    println!();
    println!("  {} {}", "f64 result: ".dimmed(), format!("{:.20}", proof.float_result).red().bold());
    println!("  {} {}", "Decimal:    ".dimmed(), proof.decimal_result.to_string().green().bold());
    println!();
    println!("  {} {}", "Discrepancy:".white().bold(), format!("{}", proof.discrepancy).bright_red().bold());
    println!();
    println!("  {}", "⚠  Float arithmetic drifts. Decimal does not.".bright_red().bold());
    println!("  {}", "   In a savings app with millions of users, this is regulatory risk.".italic().dimmed());
    println!("{}\n", "─".repeat(60).bright_red());
}

fn run_interest(
    principal: Decimal,
    rate: Decimal,
    start: NaiveDate,
    end: NaiveDate,
    compare: bool,
) {
    println!("{}", "─".repeat(60).bright_blue());
    println!("{}", "  📈  COMPOUND INTEREST — CALENDAR-AWARE".bold().white());
    println!("{}", "─".repeat(60).bright_blue());

    let result = compound_interest_calendar_aware(principal, rate, start, end);

    println!("  {}  {}", "Principal:  ".dimmed(), format!("£{:.2}", result.principal).cyan().bold());
    println!("  {}  {}", "Annual Rate:".dimmed(), format!("{}%", result.annual_rate).cyan());
    println!("  {}  {} → {}", "Period:     ".dimmed(), result.start_date.to_string().yellow(), result.end_date.to_string().yellow());
    println!("  {}  {} days", "Total Days: ".dimmed(), result.total_days.to_string().yellow());

    println!("\n{}", "  ── Year-by-Year Breakdown ──".bright_blue());
    for y in &result.year_breakdown {
        let leap = if y.days_in_year == 366 {
            " 🗓 LEAP YEAR".bright_yellow().bold().to_string()
        } else {
            "".to_string()
        };
        println!(
            "  {} {}  |  {} days/{}  |  Interest: {}  |  Balance: {}{}",
            "▶".bright_blue(),
            y.year.to_string().bold(),
            y.days_accrued,
            y.days_in_year,
            format!("£{:.4}", y.interest_this_year).green(),
            format!("£{:.4}", y.balance_end_of_year).green().bold(),
            leap
        );
    }

    println!("\n{}", "─".repeat(60).bright_blue());
    println!("  {}  {}", "Interest Earned:".white().bold(), format!("£{:.4}", result.interest_earned).green().bold());
    println!("  {}   {}", "Final Balance:  ".white().bold(), format!("£{:.4}", result.final_balance).bright_green().bold());

    if compare {
        println!("\n{}", "  ── vs Naive 365-day Calculation ──".red());
        let naive = compound_interest_naive(principal, rate, result.total_days);
        let diff = (result.final_balance - naive).abs();
        println!("  {} {}", "Naive result:".dimmed(), format!("£{:.4}", naive).red());
        println!("  {} {}", "Safe-Vault:  ".dimmed(), format!("£{:.4}", result.final_balance).green().bold());
        println!("  {} {}  ← {}", "Discrepancy: ".dimmed(), format!("£{:.4}", diff).bright_red().bold(), "silent loss per user".italic().red());
    }

    println!("{}\n", "─".repeat(60).bright_blue());
}

fn run_demo() {
    println!("\n  {}\n", "Running full Safe-Vault demonstration...".italic().dimmed());
    run_proof(1000);
    run_interest(
        dec!(50000), dec!(4.75),
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        true,
    );
}

fn print_banner() {
    println!("\n{}", "╔══════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║   🏦  S A F E - V A U L T  v1.0.0           ║".bright_blue());
    println!("{}", "║   Precision Financial Logic Engine — Rust    ║".bright_blue());
    println!("{}", "║   Zero float errors. Leap-year aware.        ║".bright_blue());
    println!("{}\n", "╚══════════════════════════════════════════════╝".bright_blue());
}

fn parse_decimal(s: &str, field: &str) -> Decimal {
    Decimal::from_str(s).unwrap_or_else(|_| {
        eprintln!("{}", format!("  ✗ Invalid {}: '{}'", field, s).red().bold());
        std::process::exit(1);
    })
}

fn parse_date(s: &str, field: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap_or_else(|_| {
        eprintln!("{}", format!("  ✗ Invalid {} date: '{}' — use YYYY-MM-DD", field, s).red().bold());
        std::process::exit(1);
    })
}