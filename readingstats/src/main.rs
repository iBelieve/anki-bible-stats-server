use clap::{Parser, Subcommand};
use readingstats::get_last_30_days_stats;
use std::process;

#[derive(Parser)]
#[command(name = "readingstats")]
#[command(about = "Analyze Bible reading statistics from KOReader databases", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show reading time for each of the last 30 days
    Daily {
        /// Path to the KOReader statistics database file
        #[arg(value_name = "DATABASE_PATH")]
        db_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Daily { db_path } => {
            run_daily_command(&db_path);
        }
    }
}

fn run_daily_command(db_path: &str) {
    match get_last_30_days_stats(db_path) {
        Ok(daily_stats) => {
            println!("\n=== DAILY READING STATS - LAST 30 DAYS ===\n");

            let total_minutes: f64 = daily_stats.iter().map(|d| d.minutes).sum();
            let avg_minutes = total_minutes / daily_stats.len() as f64;

            // Print each day
            for day in &daily_stats {
                let hours = day.minutes / 60.0;

                if day.minutes > 0.0 {
                    println!(
                        "{}: {:.2} min ({:.1} hrs)",
                        day.date, day.minutes, hours
                    );
                } else {
                    println!("{}: --- (no reading)", day.date);
                }
            }

            println!("\n--- SUMMARY ---");
            println!(
                "Total Reading Time: {:.2} minutes ({:.1} hours)",
                total_minutes,
                total_minutes / 60.0
            );
            println!(
                "Average per day: {:.2} minutes ({:.1} hours)",
                avg_minutes,
                avg_minutes / 60.0
            );

            let days_read = daily_stats.iter().filter(|d| d.minutes > 0.0).count();
            println!("Days with reading: {} out of 30", days_read);
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
