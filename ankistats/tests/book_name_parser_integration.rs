use ankistats::bible::all_books;
use ankistats::book_name_parser::try_parse_book_name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Integration test that validates the book name parser against real Bible references
/// from an Anki database.
///
/// This test reads Bible references from tests/data/bible_references.txt and
/// verifies that each reference can be successfully parsed to extract the book name.
/// It also checks that all parsed book names are valid Bible books.
///
/// To generate the test data file, run:
/// ```
/// cargo run --bin cli refs collection.anki2 > tests/data/bible_references.txt
/// ```
///
/// If the test data file doesn't exist, the test is skipped.
#[test]
fn test_parse_book_names_from_real_references() {
    let test_file_path = Path::new("tests/data/bible_references.txt");

    // Skip test if the file doesn't exist
    if !test_file_path.exists() {
        println!("Skipping test: {} not found", test_file_path.display());
        println!("To generate test data, run:");
        println!("  cargo run --bin cli refs collection.anki2 > tests/data/bible_references.txt");
        return;
    }

    // Create a set of valid Bible book names for validation
    let valid_books: HashSet<String> = all_books().map(|s| s.to_string()).collect();

    // Read the file
    let content = fs::read_to_string(test_file_path).expect("Failed to read test data file");

    let mut total_references = 0;
    let mut successful_parses = 0;
    let mut failed_parses = Vec::new();
    let mut invalid_books = Vec::new();
    let mut unique_books = HashSet::new();

    // Parse each reference
    for line in content.lines() {
        let reference = line.trim();

        // Skip empty lines
        if reference.is_empty() {
            continue;
        }

        total_references += 1;

        match try_parse_book_name(reference) {
            Ok(book_name) => {
                successful_parses += 1;
                unique_books.insert(book_name.clone());

                // Check if the parsed book name is a valid Bible book
                if !valid_books.contains(&book_name) {
                    invalid_books.push((reference.to_string(), book_name));
                }
            }
            Err(err) => {
                failed_parses.push((reference.to_string(), err));
            }
        }
    }

    // Print statistics
    println!("\n=== Book Name Parser Integration Test Results ===");
    println!("Total references tested: {}", total_references);
    println!("Successfully parsed: {}", successful_parses);
    println!("Failed to parse: {}", failed_parses.len());
    println!("Unique books found: {}", unique_books.len());

    if !failed_parses.is_empty() {
        println!("\nFailed to parse book names:");
        for (reference, err) in &failed_parses {
            println!("  - '{}': {}", reference, err);
        }
    }

    if !invalid_books.is_empty() {
        println!("\nParsed book names that don't match canonical Bible books:");
        for (reference, book_name) in &invalid_books {
            println!("  - '{}' → '{}'", reference, book_name);
        }
    }

    // The test passes if all references were successfully parsed and all book names are valid
    assert_eq!(
        failed_parses.len(),
        0,
        "Failed to parse {} out of {} references",
        failed_parses.len(),
        total_references
    );

    assert_eq!(
        invalid_books.len(),
        0,
        "Found {} invalid book names out of {} references",
        invalid_books.len(),
        total_references
    );
}
