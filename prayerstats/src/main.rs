use clap::Parser;

#[derive(Parser)]
#[command(name = "prayerstats")]
#[command(about = "Analyze prayer statistics from prayer app databases", long_about = None)]
#[command(version)]
struct Cli {
    // Future subcommands will be added here
    // Examples:
    // - Today: Show today's prayer time
    // - Daily: Show prayer time for last 30 days
    // - Weekly: Show prayer time for last 12 weeks
}

fn main() {
    let _cli = Cli::parse();

    println!("prayerstats CLI - Coming soon!");
    println!("\nThis CLI tool will provide commands for analyzing prayer statistics.");
    println!("Subcommands are not yet implemented.");
}
