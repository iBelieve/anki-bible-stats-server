use clap::{Parser, Subcommand};
use faithstats::get_faith_daily_stats;
use faithstats::models::FaithDayStatsDisplay;
use std::process;
use tabled::{Table, settings::Style};

#[derive(Parser)]
#[command(name = "faithstats")]
#[command(about = "Analyze unified faith statistics from multiple sources", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show faith statistics for each of the last 30 days
    Daily,
}

fn main() {
    // Load environment variables from .env file if present
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    match cli.command {
        Commands::Daily => {
            run_daily_command();
        }
    }
}

fn run_daily_command() {
    // Get database paths from environment variables
    let anki_db = std::env::var("ANKI_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: ANKI_DATABASE_PATH environment variable is required");
        eprintln!("Set it in a .env file or export it in your shell");
        process::exit(1);
    });

    let koreader_db = std::env::var("KOREADER_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: KOREADER_DATABASE_PATH environment variable is required");
        eprintln!("Set it in a .env file or export it in your shell");
        process::exit(1);
    });

    match get_faith_daily_stats(&anki_db, &koreader_db) {
        Ok(stats) => {
            println!("\n=== FAITH STATS - LAST 30 DAYS ===\n");

            // Convert to display format and create table
            let display_stats: Vec<FaithDayStatsDisplay> =
                stats.days.iter().map(|s| s.into()).collect();
            let table = Table::new(display_stats).with(Style::rounded()).to_string();
            println!("{}", table);

            // Print summary statistics
            println!("\n=== SUMMARY ===\n");

            println!("ANKI MEMORIZATION:");
            println!(
                "  Total: {:.2} min ({:.1} hrs)",
                stats.summary.anki_total_minutes, stats.summary.anki_total_hours
            );
            println!(
                "  Average: {:.2} min/day",
                stats.summary.anki_average_minutes_per_day
            );
            println!(
                "  Days studied: {} / {}",
                stats.summary.anki_days_studied, stats.summary.total_days
            );
            println!(
                "  Passages: +{} matured, -{} lost (net: {:+})",
                stats.summary.anki_total_matured_passages,
                stats.summary.anki_total_lost_passages,
                stats.summary.anki_net_progress
            );

            println!("\nBIBLE READING:");
            println!(
                "  Total: {:.2} min ({:.1} hrs)",
                stats.summary.reading_total_minutes, stats.summary.reading_total_hours
            );
            println!(
                "  Average: {:.2} min/day",
                stats.summary.reading_average_minutes_per_day
            );
            println!(
                "  Days read: {} / {}",
                stats.summary.reading_days_studied, stats.summary.total_days
            );

            if stats.summary.prayer_total_minutes > 0.0 {
                println!("\nPRAYER:");
                println!(
                    "  Total: {:.2} min ({:.1} hrs)",
                    stats.summary.prayer_total_minutes, stats.summary.prayer_total_hours
                );
                println!(
                    "  Average: {:.2} min/day",
                    stats.summary.prayer_average_minutes_per_day
                );
                println!(
                    "  Days prayed: {} / {}",
                    stats.summary.prayer_days_studied, stats.summary.total_days
                );
            }

            println!("\nCOMBINED TOTAL:");
            println!(
                "  Total: {:.2} min ({:.1} hrs)",
                stats.summary.total_minutes, stats.summary.total_hours
            );
            println!(
                "  Average: {:.2} min/day",
                stats.summary.average_minutes_per_day
            );
            println!(
                "  Days with any activity: {} / {}",
                stats.summary.days_with_any_activity, stats.summary.total_days
            );

            println!();
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
