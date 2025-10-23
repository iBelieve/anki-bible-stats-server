use clap::{Parser, Subcommand};
use faithstats::models::{FaithDayStatsDisplay, FaithWeekStatsDisplay};
use faithstats::{get_faith_daily_stats, get_faith_weekly_stats};
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
    /// Show faith statistics for each of the last 12 weeks
    Weekly,
}

fn main() {
    // Load environment variables from .env file if present
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    match cli.command {
        Commands::Daily => {
            run_daily_command();
        }
        Commands::Weekly => {
            run_weekly_command();
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

    let proseuche_db = std::env::var("PROSEUCHE_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: PROSEUCHE_DATABASE_PATH environment variable is required");
        eprintln!("Set it in a .env file or export it in your shell");
        process::exit(1);
    });

    match get_faith_daily_stats(&anki_db, &koreader_db, &proseuche_db) {
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

fn run_weekly_command() {
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

    let arcstats_export = std::env::var("ARCSTATS_EXPORT_PATH").unwrap_or_else(|_| {
        eprintln!("Error: ARCSTATS_EXPORT_PATH environment variable is required");
        eprintln!("Set it in a .env file or export it in your shell");
        process::exit(1);
    });

    let proseuche_db = std::env::var("PROSEUCHE_DATABASE_PATH").unwrap_or_else(|_| {
        eprintln!("Error: PROSEUCHE_DATABASE_PATH environment variable is required");
        eprintln!("Set it in a .env file or export it in your shell");
        process::exit(1);
    });

    match get_faith_weekly_stats(&anki_db, &koreader_db, &arcstats_export, &proseuche_db) {
        Ok(stats) => {
            println!("\n=== FAITH STATS - LAST 12 WEEKS ===\n");

            // Convert to display format and create table
            let display_stats: Vec<FaithWeekStatsDisplay> =
                stats.weeks.iter().map(|s| s.into()).collect();
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
                "  Average: {:.2} min/week",
                stats.summary.anki_average_minutes_per_week
            );
            println!(
                "  Weeks studied: {} / {}",
                stats.summary.anki_weeks_studied, stats.summary.total_weeks
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
                "  Average: {:.2} min/week",
                stats.summary.reading_average_minutes_per_week
            );
            println!(
                "  Weeks read: {} / {}",
                stats.summary.reading_weeks_studied, stats.summary.total_weeks
            );

            println!("\nCHURCH ATTENDANCE:");
            println!(
                "  Total: {:.2} min ({:.1} hrs)",
                stats.summary.church_total_minutes, stats.summary.church_total_hours
            );
            println!(
                "  Average: {:.2} min/week",
                stats.summary.church_average_minutes_per_week
            );
            println!(
                "  Weeks attended: {} / {}",
                stats.summary.church_weeks_attended, stats.summary.total_weeks
            );

            if stats.summary.prayer_total_minutes > 0.0 {
                println!("\nPRAYER:");
                println!(
                    "  Total: {:.2} min ({:.1} hrs)",
                    stats.summary.prayer_total_minutes, stats.summary.prayer_total_hours
                );
                println!(
                    "  Average: {:.2} min/week",
                    stats.summary.prayer_average_minutes_per_week
                );
                println!(
                    "  Weeks prayed: {} / {}",
                    stats.summary.prayer_weeks_studied, stats.summary.total_weeks
                );
            }

            println!("\nCOMBINED TOTAL:");
            println!(
                "  Total: {:.2} min ({:.1} hrs)",
                stats.summary.total_minutes, stats.summary.total_hours
            );
            println!(
                "  Average: {:.2} min/week",
                stats.summary.average_minutes_per_week
            );
            println!(
                "  Weeks with any activity: {} / {}",
                stats.summary.weeks_with_any_activity, stats.summary.total_weeks
            );

            println!();
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
