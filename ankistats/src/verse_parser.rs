/// Checks if a book name is a single-chapter book in the Bible
fn is_single_chapter_book(book_name: &str) -> bool {
    const SINGLE_CHAPTER_BOOKS: &[&str] = &["Obadiah", "Philemon", "2 John", "3 John", "Jude"];

    SINGLE_CHAPTER_BOOKS
        .iter()
        .any(|&book| book_name.eq_ignore_ascii_case(book))
}

/// Parses a Bible verse reference and counts the number of verses it contains
///
/// Supports:
/// - Single verses: "Genesis 1:1" → 1
/// - Simple ranges: "Genesis 1:1-5" → 5
/// - Verse parts (letters are stripped): "Proverbs 12:4a" → 1, "Colossians 1:9a-12" → 4
/// - Single-chapter books: "Jude 24-25" → 2 (no colon needed)
///
/// Returns an error if the reference cannot be parsed.
pub fn try_count_verses_in_reference(reference: &str) -> Result<i64, String> {
    // Strip any Unicode formatting characters (like zero-width spaces and directional marks)
    let reference = reference
        .chars()
        .filter(|c| {
            !c.is_control()
                && *c != '\u{200B}' // Zero Width Space
                && *c != '\u{FEFF}' // Zero Width No-Break Space (BOM)
                && *c != '\u{202A}' // Left-to-Right Embedding
                && *c != '\u{202B}' // Right-to-Left Embedding
                && *c != '\u{202C}' // Pop Directional Formatting
                && *c != '\u{202D}' // Left-to-Right Override
                && *c != '\u{202E}' // Right-to-Left Override
        })
        .collect::<String>();

    // Find the last colon to extract the verse portion
    let verse_part = match reference.rfind(':') {
        Some(pos) => &reference[pos + 1..],
        None => {
            // No colon found - check if this is a single-chapter book
            if let Some(space_pos) = reference.rfind(' ') {
                let book_name = &reference[..space_pos];
                if is_single_chapter_book(book_name) {
                    // Extract verse numbers after the space
                    &reference[space_pos + 1..]
                } else {
                    return Err(format!(
                        "No colon found in reference '{}' (not a single-chapter book)",
                        reference
                    ));
                }
            } else {
                return Err(format!(
                    "No colon or space found in reference '{}'",
                    reference
                ));
            }
        }
    };

    // Strip any whitespace and remaining Unicode formatting characters
    let verse_part = verse_part.trim();

    // Check if it's a range (contains a hyphen)
    if let Some(hyphen_pos) = verse_part.find('-') {
        let start_str = verse_part[..hyphen_pos].trim();
        let end_str = verse_part[hyphen_pos + 1..].trim();

        let start = parse_verse_number(start_str);
        let end = parse_verse_number(end_str);

        match (start, end) {
            (Some(s), Some(e)) if e >= s => Ok(e - s + 1),
            _ => Err(format!(
                "Could not parse range '{}' in reference '{}'",
                verse_part, reference
            )),
        }
    } else {
        // Single verse
        match parse_verse_number(verse_part) {
            Some(_) => Ok(1),
            None => Err(format!(
                "Could not parse verse '{}' in reference '{}'",
                verse_part, reference
            )),
        }
    }
}

/// Parses a Bible verse reference and counts the number of verses it contains
///
/// Supports:
/// - Single verses: "Genesis 1:1" → 1
/// - Simple ranges: "Genesis 1:1-5" → 5
/// - Verse parts (letters are stripped): "Proverbs 12:4a" → 1, "Colossians 1:9a-12" → 4
///
/// For unparsable references, logs a warning and returns 1 (treating as a single verse).
/// This is a wrapper around `try_count_verses_in_reference` for use in contexts where
/// errors should be handled gracefully (e.g., SQLite functions).
pub fn count_verses_in_reference(reference: &str) -> i64 {
    match try_count_verses_in_reference(reference) {
        Ok(count) => count,
        Err(err) => {
            eprintln!("Warning: {}, treating as 1 verse", err);
            1
        }
    }
}

/// Parses a verse number, stripping any letter suffixes (e.g., "4a" → 4)
fn parse_verse_number(s: &str) -> Option<i64> {
    // Strip any letters from the end (a, b, c, etc.)
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();

    digits.parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_verse() {
        assert_eq!(count_verses_in_reference("Genesis 1:1"), 1);
        assert_eq!(count_verses_in_reference("2 Timothy 3:16"), 1);
        assert_eq!(count_verses_in_reference("Psalm 119:105"), 1);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("Genesis 1:1"), Ok(1));
        assert_eq!(try_count_verses_in_reference("2 Timothy 3:16"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Psalm 119:105"), Ok(1));
    }

    #[test]
    fn test_simple_range() {
        assert_eq!(count_verses_in_reference("Genesis 1:1-5"), 5);
        assert_eq!(count_verses_in_reference("Romans 5:1-8"), 8);
        assert_eq!(count_verses_in_reference("John 3:16-17"), 2);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("Genesis 1:1-5"), Ok(5));
        assert_eq!(try_count_verses_in_reference("Romans 5:1-8"), Ok(8));
        assert_eq!(try_count_verses_in_reference("John 3:16-17"), Ok(2));
    }

    #[test]
    fn test_verse_with_letter_suffix() {
        assert_eq!(count_verses_in_reference("Proverbs 12:4a"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:1b"), 1);
        assert_eq!(count_verses_in_reference("Matthew 5:3a"), 1);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("Proverbs 12:4a"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Genesis 1:1b"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Matthew 5:3a"), Ok(1));
    }

    #[test]
    fn test_range_with_letter_suffix() {
        assert_eq!(count_verses_in_reference("Colossians 1:9a-12"), 4);
        assert_eq!(count_verses_in_reference("Genesis 1:1b-3"), 3);
        assert_eq!(count_verses_in_reference("Romans 5:1a-5b"), 5);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("Colossians 1:9a-12"), Ok(4));
        assert_eq!(try_count_verses_in_reference("Genesis 1:1b-3"), Ok(3));
        assert_eq!(try_count_verses_in_reference("Romans 5:1a-5b"), Ok(5));
    }

    #[test]
    fn test_single_verse_range() {
        assert_eq!(count_verses_in_reference("John 3:16-16"), 1);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("John 3:16-16"), Ok(1));
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(count_verses_in_reference("Genesis 1: 1"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:1 - 5"), 5);
        assert_eq!(count_verses_in_reference("Romans 5: 1 - 8 "), 8);

        // Test the Result-returning version
        assert_eq!(try_count_verses_in_reference("Genesis 1: 1"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Genesis 1:1 - 5"), Ok(5));
        assert_eq!(try_count_verses_in_reference("Romans 5: 1 - 8 "), Ok(8));
    }

    #[test]
    fn test_parse_verse_number() {
        assert_eq!(parse_verse_number("1"), Some(1));
        assert_eq!(parse_verse_number("12"), Some(12));
        assert_eq!(parse_verse_number("4a"), Some(4));
        assert_eq!(parse_verse_number("16b"), Some(16));
        assert_eq!(parse_verse_number("105"), Some(105));
    }

    #[test]
    fn test_invalid_references_fallback_to_one() {
        // count_verses_in_reference should return 1 for invalid references
        assert_eq!(count_verses_in_reference("Genesis 1"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:abc"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:5-1"), 1);

        // try_count_verses_in_reference should return errors
        assert!(try_count_verses_in_reference("Genesis 1").is_err());
        assert!(try_count_verses_in_reference("Genesis 1:abc").is_err());
        assert!(try_count_verses_in_reference("Genesis 1:5-1").is_err());
    }

    #[test]
    fn test_is_single_chapter_book() {
        // These are the five single-chapter books in the Bible
        assert!(is_single_chapter_book("Obadiah"));
        assert!(is_single_chapter_book("Philemon"));
        assert!(is_single_chapter_book("2 John"));
        assert!(is_single_chapter_book("3 John"));
        assert!(is_single_chapter_book("Jude"));

        // Case insensitive
        assert!(is_single_chapter_book("jude"));
        assert!(is_single_chapter_book("PHILEMON"));

        // Multi-chapter books should return false
        assert!(!is_single_chapter_book("Genesis"));
        assert!(!is_single_chapter_book("Matthew"));
        assert!(!is_single_chapter_book("1 John"));
        assert!(!is_single_chapter_book("2 Corinthians"));
    }

    #[test]
    fn test_single_chapter_books() {
        // Single-chapter books without colons
        assert_eq!(try_count_verses_in_reference("Jude 24-25"), Ok(2));
        assert_eq!(try_count_verses_in_reference("Jude 24"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Philemon 1"), Ok(1));
        assert_eq!(try_count_verses_in_reference("3 John 14"), Ok(1));
        assert_eq!(try_count_verses_in_reference("Obadiah 1"), Ok(1));
        assert_eq!(try_count_verses_in_reference("2 John 1-3"), Ok(3));

        // Also test the wrapper function
        assert_eq!(count_verses_in_reference("Jude 24-25"), 2);
        assert_eq!(count_verses_in_reference("Jude 24"), 1);
        assert_eq!(count_verses_in_reference("Philemon 1"), 1);
    }

    #[test]
    fn test_unicode_formatting_characters() {
        // Test with various Unicode formatting characters (using escaped sequences)
        assert_eq!(
            try_count_verses_in_reference("Psalm \u{202d}51\u{202c}:\u{202d}3"),
            Ok(1)
        );
        assert_eq!(
            try_count_verses_in_reference("Genesis\u{202d} \u{202d}1\u{202c}:\u{202d}1"),
            Ok(1)
        );

        // Also test the wrapper function
        assert_eq!(
            count_verses_in_reference("Psalm \u{202d}51\u{202c}:\u{202d}3"),
            1
        );
    }
}
