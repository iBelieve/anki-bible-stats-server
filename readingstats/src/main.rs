use clap::Parser;

#[derive(Parser)]
#[command(name = "readingstats")]
#[command(about = "Analyze Bible reading statistics from KOReader databases", long_about = None)]
#[command(version)]
struct Cli {
    // Future subcommands will be added here
    // Examples:
    // - Books: Show statistics for each Bible book read
    // - Today: Show today's reading time
    // - Daily: Show reading time for last 30 days
    // - Weekly: Show reading time for last 12 weeks
}

fn main() {
    let _cli = Cli::parse();

    println!("readingstats CLI - Coming soon!");
    println!("\nThis CLI tool will provide commands for analyzing Bible reading statistics.");
    println!("Subcommands are not yet implemented.");
}
