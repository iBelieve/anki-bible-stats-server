use arcstats::{load_metadata, load_places_file, load_items_for_month};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "arcstats")]
#[command(about = "Load and parse Arc Timeline export data", long_about = None)]
struct Args {
    /// Path to the Arc export directory
    export_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let export_path = &args.export_path;

    println!("Loading Arc export from: {:?}\n", export_path);

    // Load metadata
    println!("=== Loading Metadata ===");
    match load_metadata(export_path) {
        Ok(metadata) => {
            println!("✓ Metadata loaded successfully");
            println!("  Schema version: {}", metadata.schema_version);
            println!("  Export type: {}", metadata.export_type);
            println!("  Items: {}", metadata.stats.item_count);
            println!("  Places: {}", metadata.stats.place_count);
            println!("  Samples: {}", metadata.stats.sample_count);
        }
        Err(e) => {
            println!("✗ Failed to load metadata: {}", e);
        }
    }
    println!();

    // Load places
    println!("=== Loading Places ===");
    let place_chars = "0123456789ABCDEF";
    let mut total_places = 0;
    let mut failed_place_files = Vec::new();

    for c in place_chars.chars() {
        match load_places_file(export_path, c) {
            Ok(places) => {
                if !places.is_empty() {
                    println!("✓ Loaded {}.json: {} places", c, places.len());
                    total_places += places.len();
                }
            }
            Err(e) => {
                failed_place_files.push((c, e.to_string()));
            }
        }
    }

    if !failed_place_files.is_empty() {
        println!("\nFailed place files:");
        for (c, err) in failed_place_files {
            println!("✗ {}.json: {}", c, err);
        }
    }
    println!("\nTotal places loaded: {}", total_places);
    println!();

    // Load items
    println!("=== Loading Items ===");
    let items_dir = export_path.join("items");

    match fs::read_dir(&items_dir) {
        Ok(entries) => {
            let mut month_files: Vec<String> = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        if filename.ends_with(".json") {
                            let year_month = filename.trim_end_matches(".json");
                            month_files.push(year_month.to_string());
                        }
                    }
                }
            }

            month_files.sort();

            let mut total_items = 0;
            let mut failed_months = Vec::new();

            for year_month in month_files {
                match load_items_for_month(export_path, &year_month) {
                    Ok(items) => {
                        println!("✓ Loaded {}.json: {} items", year_month, items.len());
                        total_items += items.len();
                    }
                    Err(e) => {
                        println!("✗ Failed to load {}.json", year_month);
                        failed_months.push((year_month, e));
                    }
                }
            }

            if !failed_months.is_empty() {
                println!("\n=== Detailed Errors ===");
                for (month, err) in failed_months {
                    println!("\n{}.json:", month);
                    println!("{:#}", err);
                }
            }

            println!("\nTotal items loaded: {}", total_items);
        }
        Err(e) => {
            println!("✗ Failed to read items directory: {}", e);
        }
    }
}
