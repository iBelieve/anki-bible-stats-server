/// Normalizes a book name to use the standard display name
///
/// Currently handles:
/// - "Psalm" (from references) → "Psalms" (display name)
fn normalize_book_name(book_name: &str) -> String {
    if book_name.eq_ignore_ascii_case("Psalm") {
        "Psalms".to_string()
    } else {
        book_name.to_string()
    }
}

/// Parses a Bible reference and extracts the book name
///
/// Supports:
/// - Multi-chapter books: "Genesis 1:1" → "Genesis"
/// - Numbered books: "2 Timothy 3:16" → "2 Timothy"
/// - Single-chapter books: "Jude 24" → "Jude"
///
/// Returns an error if the reference cannot be parsed.
pub fn try_parse_book_name(reference: &str) -> Result<String, String> {
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

    // Find the last space to extract the book name
    match reference.rfind(' ') {
        Some(pos) => {
            let book_name = reference[..pos].trim();
            if book_name.is_empty() {
                Err(format!("No book name found in reference '{}'", reference))
            } else {
                Ok(normalize_book_name(book_name))
            }
        }
        None => Err(format!(
            "No space found in reference '{}' (cannot extract book name)",
            reference
        )),
    }
}

/// Parses a Bible reference and extracts the book name
///
/// Supports:
/// - Multi-chapter books: "Genesis 1:1" → "Genesis"
/// - Numbered books: "2 Timothy 3:16" → "2 Timothy"
/// - Single-chapter books: "Jude 24" → "Jude"
///
/// For unparsable references, returns None.
/// This is a wrapper around `try_parse_book_name` for use in contexts where
/// errors should be handled gracefully (e.g., SQLite functions).
pub fn parse_book_name(reference: &str) -> Option<String> {
    match try_parse_book_name(reference) {
        Ok(book_name) => Some(book_name),
        Err(err) => {
            eprintln!("Warning: {}", err);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_book_name_multi_chapter() {
        // Multi-chapter books
        assert_eq!(
            try_parse_book_name("Genesis 1:1"),
            Ok("Genesis".to_string())
        );
        assert_eq!(
            try_parse_book_name("Psalm 119:105"),
            Ok("Psalms".to_string())
        );
        assert_eq!(try_parse_book_name("John 3:16"), Ok("John".to_string()));
        assert_eq!(
            try_parse_book_name("Romans 5:1-8"),
            Ok("Romans".to_string())
        );

        // Test the wrapper function
        assert_eq!(parse_book_name("Genesis 1:1"), Some("Genesis".to_string()));
        assert_eq!(parse_book_name("Psalm 119:105"), Some("Psalms".to_string()));
    }

    #[test]
    fn test_parse_book_name_numbered_books() {
        // Numbered books
        assert_eq!(
            try_parse_book_name("1 Samuel 17:47"),
            Ok("1 Samuel".to_string())
        );
        assert_eq!(
            try_parse_book_name("2 Timothy 3:16"),
            Ok("2 Timothy".to_string())
        );
        assert_eq!(
            try_parse_book_name("1 Corinthians 13:4-7"),
            Ok("1 Corinthians".to_string())
        );
        assert_eq!(try_parse_book_name("3 John 14"), Ok("3 John".to_string()));

        // Test the wrapper function
        assert_eq!(
            parse_book_name("2 Timothy 3:16"),
            Some("2 Timothy".to_string())
        );
    }

    #[test]
    fn test_parse_book_name_single_chapter_books() {
        // Single-chapter books (no colon)
        assert_eq!(try_parse_book_name("Jude 24"), Ok("Jude".to_string()));
        assert_eq!(try_parse_book_name("Jude 24-25"), Ok("Jude".to_string()));
        assert_eq!(
            try_parse_book_name("Philemon 1"),
            Ok("Philemon".to_string())
        );
        assert_eq!(try_parse_book_name("Obadiah 1"), Ok("Obadiah".to_string()));

        // Test the wrapper function
        assert_eq!(parse_book_name("Jude 24-25"), Some("Jude".to_string()));
    }

    #[test]
    fn test_parse_book_name_with_verse_letters() {
        // Verse references with letter suffixes
        assert_eq!(
            try_parse_book_name("Proverbs 12:4a"),
            Ok("Proverbs".to_string())
        );
        assert_eq!(
            try_parse_book_name("Colossians 1:9a-12"),
            Ok("Colossians".to_string())
        );
        assert_eq!(try_parse_book_name("Acts 22:16b"), Ok("Acts".to_string()));

        // Test the wrapper function
        assert_eq!(
            parse_book_name("Proverbs 12:4a"),
            Some("Proverbs".to_string())
        );
    }

    #[test]
    fn test_parse_book_name_with_unicode() {
        // Test with Unicode formatting characters (using escaped sequences)
        assert_eq!(
            try_parse_book_name("Psalm \u{202d}51\u{202c}:\u{202d}3"),
            Ok("Psalms".to_string())
        );
        assert_eq!(
            try_parse_book_name("Ephesians\u{202c} \u{202d}4:32\u{202c}"),
            Ok("Ephesians".to_string())
        );

        // Test the wrapper function
        assert_eq!(
            parse_book_name("Psalm \u{202d}51\u{202c}:\u{202d}3"),
            Some("Psalms".to_string())
        );
    }

    #[test]
    fn test_parse_book_name_invalid() {
        // References without spaces should fail
        assert!(try_parse_book_name("Genesis").is_err());
        assert!(try_parse_book_name("").is_err());

        // Test the wrapper function returns None
        assert_eq!(parse_book_name("Genesis"), None);
        assert_eq!(parse_book_name(""), None);
    }
}
