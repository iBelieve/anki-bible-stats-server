/// Parses a Bible verse reference and counts the number of verses it contains
///
/// Supports:
/// - Single verses: "Genesis 1:1" → 1
/// - Simple ranges: "Genesis 1:1-5" → 5
/// - Verse parts (letters are stripped): "Proverbs 12:4a" → 1, "Colossians 1:9a-12" → 4
///
/// For unparsable references, logs an error and returns 1 (treating as a single verse).
pub fn count_verses_in_reference(reference: &str) -> i64 {
    // Find the last colon to extract the verse portion
    let verse_part = match reference.rfind(':') {
        Some(pos) => &reference[pos + 1..],
        None => {
            eprintln!(
                "Warning: No colon found in reference '{}', treating as 1 verse",
                reference
            );
            return 1;
        }
    };

    // Strip any whitespace
    let verse_part = verse_part.trim();

    // Check if it's a range (contains a hyphen)
    if let Some(hyphen_pos) = verse_part.find('-') {
        let start_str = verse_part[..hyphen_pos].trim();
        let end_str = verse_part[hyphen_pos + 1..].trim();

        let start = parse_verse_number(start_str);
        let end = parse_verse_number(end_str);

        match (start, end) {
            (Some(s), Some(e)) if e >= s => e - s + 1,
            _ => {
                eprintln!(
                    "Warning: Could not parse range '{}' in reference '{}', treating as 1 verse",
                    verse_part, reference
                );
                1
            }
        }
    } else {
        // Single verse
        match parse_verse_number(verse_part) {
            Some(_) => 1,
            None => {
                eprintln!(
                    "Warning: Could not parse verse '{}' in reference '{}', treating as 1 verse",
                    verse_part, reference
                );
                1
            }
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
    }

    #[test]
    fn test_simple_range() {
        assert_eq!(count_verses_in_reference("Genesis 1:1-5"), 5);
        assert_eq!(count_verses_in_reference("Romans 5:1-8"), 8);
        assert_eq!(count_verses_in_reference("John 3:16-17"), 2);
    }

    #[test]
    fn test_verse_with_letter_suffix() {
        assert_eq!(count_verses_in_reference("Proverbs 12:4a"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:1b"), 1);
        assert_eq!(count_verses_in_reference("Matthew 5:3a"), 1);
    }

    #[test]
    fn test_range_with_letter_suffix() {
        assert_eq!(count_verses_in_reference("Colossians 1:9a-12"), 4);
        assert_eq!(count_verses_in_reference("Genesis 1:1b-3"), 3);
        assert_eq!(count_verses_in_reference("Romans 5:1a-5b"), 5);
    }

    #[test]
    fn test_single_verse_range() {
        assert_eq!(count_verses_in_reference("John 3:16-16"), 1);
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(count_verses_in_reference("Genesis 1: 1"), 1);
        assert_eq!(count_verses_in_reference("Genesis 1:1 - 5"), 5);
        assert_eq!(count_verses_in_reference("Romans 5: 1 - 8 "), 8);
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
        // No colon
        assert_eq!(count_verses_in_reference("Genesis 1"), 1);

        // Invalid verse numbers
        assert_eq!(count_verses_in_reference("Genesis 1:abc"), 1);

        // Invalid range
        assert_eq!(count_verses_in_reference("Genesis 1:5-1"), 1);
    }
}
