use ankistats::verse_parser::try_count_verses_in_reference;
use std::fs;
use std::path::Path;

/// Integration test that validates the verse parser against real Bible references
/// from an Anki database.
///
/// This test reads Bible references from tests/data/bible_references.txt and
/// verifies that each reference can be successfully parsed.
///
/// To generate the test data file, run:
/// ```
/// cargo run --bin cli refs collection.anki2 > tests/data/bible_references.txt
/// ```
///
/// If the test data file doesn't exist, the test is skipped.
#[test]
fn test_parse_real_bible_references() {
    let test_file_path = Path::new("tests/data/bible_references.txt");

    // Skip test if the file doesn't exist
    if !test_file_path.exists() {
        println!("Skipping test: {} not found", test_file_path.display());
        println!("To generate test data, run:");
        println!("  cargo run --bin cli refs collection.anki2 > tests/data/bible_references.txt");
        return;
    }

    // Read the file
    let content = fs::read_to_string(test_file_path).expect("Failed to read test data file");

    let mut total_references = 0;
    let mut successful_parses = 0;
    let mut failed_parses = Vec::new();
    let mut total_verses = 0i64;

    // Parse each reference
    for line in content.lines() {
        let reference = line.trim();

        // Skip empty lines
        if reference.is_empty() {
            continue;
        }

        total_references += 1;

        match try_count_verses_in_reference(reference) {
            Ok(count) => {
                successful_parses += 1;
                total_verses += count;

                // Sanity check: count should be at least 1
                assert!(
                    count >= 1,
                    "Reference '{}' returned invalid count: {}",
                    reference,
                    count
                );
            }
            Err(err) => {
                failed_parses.push((reference.to_string(), err));
            }
        }
    }

    // Print statistics
    println!("\n=== Verse Parser Integration Test Results ===");
    println!("Total references tested: {}", total_references);
    println!("Successfully parsed: {}", successful_parses);
    println!("Failed to parse: {}", failed_parses.len());
    println!("Total verses counted: {}", total_verses);

    if !failed_parses.is_empty() {
        println!("\nFailed references:");
        for (reference, err) in &failed_parses {
            println!("  - '{}': {}", reference, err);
        }
    }

    // The test passes if all references were successfully parsed
    assert_eq!(
        failed_parses.len(),
        0,
        "Failed to parse {} out of {} references",
        failed_parses.len(),
        total_references
    );
}
