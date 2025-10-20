use ankistats::models::{BookStats, BookStatsDisplay};
use ankistats::{
    get_bible_references, get_bible_stats, get_last_12_weeks_stats, get_last_30_days_stats,
    get_today_study_time,
};
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
    /// Show study time for each of the last 12 weeks
    Weekly {
        /// Path to the Anki database file
        #[arg(value_name = "DATABASE_PATH")]
        db_path: String,
    },
    /// List all Bible references in the database
    Refs {
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
        Commands::Weekly { db_path } => {
            run_weekly_command(&db_path);
        }
        Commands::Refs { db_path } => {
            run_refs_command(&db_path);
        }
    }
}

fn run_books_command(db_path: &str) {
    match get_bible_stats(db_path) {
        Ok(stats) => {
            println!("\n=== OLD TESTAMENT ===\n");
            print_book_stats(&stats.old_testament.book_stats);
            println!(
                "\nOT Passages: Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.old_testament.mature_passages,
                stats.old_testament.young_passages,
                stats.old_testament.unseen_passages,
                stats.old_testament.suspended_passages,
                stats.old_testament.total_passages()
            );
            println!(
                "OT Verses:   Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.old_testament.mature_verses,
                stats.old_testament.young_verses,
                stats.old_testament.unseen_verses,
                stats.old_testament.suspended_verses,
                stats.old_testament.total_verses()
            );

            println!("\n\n=== NEW TESTAMENT ===\n");
            print_book_stats(&stats.new_testament.book_stats);
            println!(
                "\nNT Passages: Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.new_testament.mature_passages,
                stats.new_testament.young_passages,
                stats.new_testament.unseen_passages,
                stats.new_testament.suspended_passages,
                stats.new_testament.total_passages()
            );
            println!(
                "NT Verses:   Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.new_testament.mature_verses,
                stats.new_testament.young_verses,
                stats.new_testament.unseen_verses,
                stats.new_testament.suspended_verses,
                stats.new_testament.total_verses()
            );

            println!("\n\n=== GRAND TOTAL ===");
            println!(
                "Passages: Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.total_mature_passages(),
                stats.total_young_passages(),
                stats.total_unseen_passages(),
                stats.total_suspended_passages(),
                stats.total_passages()
            );
            println!(
                "Verses:   Mature={}, Young={}, Unseen={}, Suspended={}, Total={}",
                stats.total_mature_verses(),
                stats.total_young_verses(),
                stats.total_unseen_verses(),
                stats.total_suspended_verses(),
                stats.total_verses()
            );
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}

fn print_book_stats(book_stats: &[BookStats]) {
    let display_stats: Vec<BookStatsDisplay> = book_stats.iter().map(|s| s.into()).collect();
    let table = Table::new(display_stats).with(Style::rounded()).to_string();
    println!("{}", table);
    println!("\n(Format: Passages / Verses)");
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
    match get_last_30_days_stats(db_path) {
        Ok(daily_stats) => {
            println!("\n=== DAILY STATS - LAST 30 DAYS ===\n");

            let total_minutes: f64 = daily_stats.iter().map(|d| d.minutes).sum();
            let avg_minutes = total_minutes / daily_stats.len() as f64;
            let total_matured: i64 = daily_stats.iter().map(|d| d.matured_passages).sum();
            let total_lost: i64 = daily_stats.iter().map(|d| d.lost_passages).sum();

            // Print each day
            for day in &daily_stats {
                let hours = day.minutes / 60.0;
                let progress_str = if day.matured_passages > 0 || day.lost_passages > 0 {
                    format!(
                        " | Matured: {}, Lost: {}, Cumulative: {}",
                        day.matured_passages, day.lost_passages, day.cumulative_passages
                    )
                } else if day.cumulative_passages != 0 {
                    format!(" | Cumulative: {}", day.cumulative_passages)
                } else {
                    String::new()
                };

                if day.minutes > 0.0 || day.matured_passages > 0 || day.lost_passages > 0 {
                    println!(
                        "{}: {:.2} min ({:.1} hrs){}",
                        day.date, day.minutes, hours, progress_str
                    );
                } else {
                    println!("{}: --- (no activity)", day.date);
                }
            }

            println!("\n--- SUMMARY ---");
            println!(
                "Study Time: {:.2} minutes ({:.1} hours)",
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

            println!("\nProgress:");
            println!("  Matured: {} passages", total_matured);
            println!("  Lost: {} passages", total_lost);
            println!("  Net: {} passages", total_matured - total_lost);
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}

fn run_weekly_command(db_path: &str) {
    match get_last_12_weeks_stats(db_path) {
        Ok(weekly_stats) => {
            println!("\n=== WEEKLY STATS - LAST 12 WEEKS ===\n");

            let total_minutes: f64 = weekly_stats.iter().map(|w| w.minutes).sum();
            let avg_minutes = total_minutes / weekly_stats.len() as f64;
            let total_matured: i64 = weekly_stats.iter().map(|w| w.matured_passages).sum();
            let total_lost: i64 = weekly_stats.iter().map(|w| w.lost_passages).sum();

            // Print each week
            for week in &weekly_stats {
                let hours = week.minutes / 60.0;
                let progress_str = if week.matured_passages > 0 || week.lost_passages > 0 {
                    format!(
                        " | Matured: {}, Lost: {}, Cumulative: {}",
                        week.matured_passages, week.lost_passages, week.cumulative_passages
                    )
                } else if week.cumulative_passages != 0 {
                    format!(" | Cumulative: {}", week.cumulative_passages)
                } else {
                    String::new()
                };

                if week.minutes > 0.0 || week.matured_passages > 0 || week.lost_passages > 0 {
                    println!(
                        "Week of {}: {:.2} min ({:.1} hrs){}",
                        week.week_start, week.minutes, hours, progress_str
                    );
                } else {
                    println!("Week of {}: --- (no activity)", week.week_start);
                }
            }

            println!("\n--- SUMMARY ---");
            println!(
                "Study Time: {:.2} minutes ({:.1} hours)",
                total_minutes,
                total_minutes / 60.0
            );
            println!(
                "Average per week: {:.2} minutes ({:.1} hours)",
                avg_minutes,
                avg_minutes / 60.0
            );

            let weeks_studied = weekly_stats.iter().filter(|w| w.minutes > 0.0).count();
            println!("Weeks studied: {} out of 12", weeks_studied);

            println!("\nProgress:");
            println!("  Matured: {} passages", total_matured);
            println!("  Lost: {} passages", total_lost);
            println!("  Net: {} passages", total_matured - total_lost);
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}

fn run_refs_command(db_path: &str) {
    match get_bible_references(db_path) {
        Ok(references) => {
            for reference in references {
                println!("{}", reference);
            }
        }
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
