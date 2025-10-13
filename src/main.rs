use anki_bible_stats::get_bible_stats;
use anki_bible_stats::models::BookStats;
use clap::{Parser, Subcommand};
use std::process;
use tabled::{settings::Style, Table};

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Books { db_path } => {
            run_books_command(&db_path);
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
