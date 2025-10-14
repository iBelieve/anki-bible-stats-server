use anki_bible_stats::models::BookStats;
use anki_bible_stats::{get_bible_stats, get_study_time_last_30_days, get_today_study_time};
use clap::{Parser, Subcommand};
use std::process;
use tabled::{Table, settings::Style};

#[derive(Parser)]
#[command(name = "anki-bible-stats")]
#[command(about = "Analyze Anki flashcard databases for Bible verse memorization progress", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show statistics for each Bible book
    Books {
        /// Path to the Anki database file
        #[arg(value_name = "DATABASE_PATH")]
        db_path: String,
    },
    /// Show study time for today
    Today {
        /// Path to the Anki database file
        #[arg(value_name = "DATABASE_PATH")]
        db_path: String,
    },
    /// Show study time for each of the last 30 days
    Daily {
        /// Path to the Anki database file
        #[arg(value_name = "DATABASE_PATH")]
        db_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Books { db_path } => {
            run_books_command(&db_path);
        }
        Commands::Today { db_path } => {
            run_today_command(&db_path);
        }
        Commands::Daily { db_path } => {
            run_daily_command(&db_path);
        }
    }
}

fn run_books_command(db_path: &str) {
    match get_bible_stats(db_path) {
        Ok(stats) => {
            println!("\n=== OLD TESTAMENT ===\n");
            print_book_stats(&stats.old_testament.book_stats);
            println!(
                "\nOT Totals: Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.old_testament.mature_count,
                stats.old_testament.young_count,
                stats.old_testament.unseen_count,
                stats.old_testament.suspended_count,
                stats.old_testament.total_cards()
            );

            println!("\n\n=== NEW TESTAMENT ===\n");
            print_book_stats(&stats.new_testament.book_stats);
            println!(
                "\nNT Totals: Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.new_testament.mature_count,
                stats.new_testament.young_count,
                stats.new_testament.unseen_count,
                stats.new_testament.suspended_count,
                stats.new_testament.total_cards()
            );

            println!("\n\n=== GRAND TOTAL ===");
            println!(
                "Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.total_mature(),
                stats.total_young(),
                stats.total_unseen(),
                stats.total_suspended(),
                stats.total_cards()
            );
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}

fn print_book_stats(book_stats: &[BookStats]) {
    let table = Table::new(book_stats).with(Style::rounded()).to_string();
    println!("{}", table);
}

fn run_today_command(db_path: &str) {
    match get_today_study_time(db_path) {
        Ok(minutes) => {
            println!("\n=== TODAY'S STUDY TIME ===\n");
            println!(
                "Total: {:.2} minutes ({:.1} hours)",
                minutes,
                minutes / 60.0
            );
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}

fn run_daily_command(db_path: &str) {
    match get_study_time_last_30_days(db_path) {
        Ok(daily_stats) => {
            println!("\n=== STUDY TIME - LAST 30 DAYS ===\n");

            let total_minutes: f64 = daily_stats.iter().map(|d| d.minutes).sum();
            let avg_minutes = total_minutes / daily_stats.len() as f64;

            // Print each day
            for day in &daily_stats {
                let hours = day.minutes / 60.0;
                if day.minutes > 0.0 {
                    println!("{}: {:.2} min ({:.1} hrs)", day.date, day.minutes, hours);
                } else {
                    println!("{}: --- (no study)", day.date);
                }
            }

            println!("\n--- SUMMARY ---");
            println!(
                "Total: {:.2} minutes ({:.1} hours)",
                total_minutes,
                total_minutes / 60.0
            );
            println!(
                "Average per day: {:.2} minutes ({:.1} hours)",
                avg_minutes,
                avg_minutes / 60.0
            );

            let days_studied = daily_stats.iter().filter(|d| d.minutes > 0.0).count();
            println!("Days studied: {} out of 30", days_studied);
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
