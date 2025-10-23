use crate::models::{Item, ItemWithPlace, Metadata, Place};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Cache for places, shared via Rc
#[derive(Debug, Clone)]
pub struct PlaceCache {
    places: HashMap<String, Rc<Place>>,
    export_path: PathBuf,
}

impl PlaceCache {
    /// Create a new empty place cache
    pub fn new<P: AsRef<Path>>(export_path: P) -> Self {
        Self {
            places: HashMap::new(),
            export_path: export_path.as_ref().to_path_buf(),
        }
    }

    /// Get a place by ID, loading it if necessary
    pub fn get_place(&mut self, place_id: &str) -> Result<Rc<Place>> {
        // Check if already cached
        if let Some(place) = self.places.get(place_id) {
            return Ok(Rc::clone(place));
        }

        // Load the place file based on first character of ID
        let first_char = place_id
            .chars()
            .next()
            .context("Place ID is empty")?
            .to_ascii_uppercase();

        let places = load_places_file(&self.export_path, first_char)?;

        // Cache all places from this file
        for place in places {
            let place_rc = Rc::new(place);
            self.places
                .insert(place_rc.id.clone(), Rc::clone(&place_rc));
        }

        // Now retrieve the requested place
        self.places
            .get(place_id)
            .map(Rc::clone)
            .context(format!("Place ID {} not found in file", place_id))
    }

    /// Get number of cached places
    pub fn len(&self) -> usize {
        self.places.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.places.is_empty()
    }
}

/// Load metadata from export directory
pub fn load_metadata<P: AsRef<Path>>(export_path: P) -> Result<Metadata> {
    let metadata_path = export_path.as_ref().join("metadata.json");
    let content = fs::read_to_string(&metadata_path)
        .context(format!("Failed to read metadata file: {:?}", metadata_path))?;

    serde_json::from_str(&content).context("Failed to parse metadata JSON")
}

/// Load a single place file by its first character (0-9, A-F)
pub fn load_places_file<P: AsRef<Path>>(export_path: P, first_char: char) -> Result<Vec<Place>> {
    let filename = format!("{}.json", first_char);
    let places_path = export_path.as_ref().join("places").join(&filename);

    let content = fs::read_to_string(&places_path)
        .context(format!("Failed to read places file: {:?}", places_path))?;

    serde_json::from_str(&content).context(format!("Failed to parse places file: {}", filename))
}

/// Load all places from all files (0-9, A-F)
pub fn load_all_places<P: AsRef<Path>>(export_path: P) -> Result<Vec<Place>> {
    let mut all_places = Vec::new();
    let place_chars = "0123456789ABCDEF";

    for c in place_chars.chars() {
        // Some files might not exist, so we skip them
        match load_places_file(&export_path, c) {
            Ok(mut places) => all_places.append(&mut places),
            Err(_) => continue, // File doesn't exist, skip
        }
    }

    Ok(all_places)
}

/// Load items for a specific month (e.g., "2025-08")
pub fn load_items_for_month<P: AsRef<Path>>(export_path: P, year_month: &str) -> Result<Vec<Item>> {
    let filename = format!("{}.json", year_month);
    let items_path = export_path.as_ref().join("items").join(&filename);

    let content = fs::read_to_string(&items_path)
        .context(format!("Failed to read items file: {:?}", items_path))?;

    serde_json::from_str(&content).context(format!("Failed to parse items file: {}", filename))
}

/// Load all items from all available month files
pub fn load_all_items<P: AsRef<Path>>(export_path: P) -> Result<Vec<Item>> {
    let items_dir = export_path.as_ref().join("items");
    let mut all_items = Vec::new();

    // Read the items directory
    let entries = fs::read_dir(&items_dir)
        .context(format!("Failed to read items directory: {:?}", items_dir))?;

    // Collect month files and sort them
    let mut month_files: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if filename.ends_with(".json") {
                    // Extract year-month part (e.g., "2025-08" from "2025-08.json")
                    let year_month = filename.trim_end_matches(".json");
                    month_files.push(year_month.to_string());
                }
            }
        }
    }

    // Sort chronologically
    month_files.sort();

    // Load each month
    for year_month in month_files {
        let mut items = load_items_for_month(&export_path, &year_month)?;
        all_items.append(&mut items);
    }

    Ok(all_items)
}

/// Load items with their associated places resolved
pub fn load_items_with_places<P: AsRef<Path>>(
    export_path: P,
    year_month: &str,
) -> Result<Vec<ItemWithPlace>> {
    let items = load_items_for_month(&export_path, year_month)?;
    let mut place_cache = PlaceCache::new(&export_path);
    let mut items_with_places = Vec::new();

    for item in items {
        let place = if let Some(place_id) = item.place_id() {
            Some(place_cache.get_place(place_id)?)
        } else {
            None
        };

        items_with_places.push(ItemWithPlace { item, place });
    }

    Ok(items_with_places)
}

/// Load all items with their associated places resolved
pub fn load_all_items_with_places<P: AsRef<Path>>(
    export_path: P,
) -> Result<Vec<ItemWithPlace>> {
    let items = load_all_items(&export_path)?;
    let mut place_cache = PlaceCache::new(&export_path);
    let mut items_with_places = Vec::new();

    for item in items {
        let place = if let Some(place_id) = item.place_id() {
            Some(place_cache.get_place(place_id)?)
        } else {
            None
        };

        items_with_places.push(ItemWithPlace { item, place });
    }

    Ok(items_with_places)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPORT_PATH: &str = "export";

    #[test]
    fn test_load_metadata() {
        let metadata = load_metadata(EXPORT_PATH).expect("Failed to load metadata");
        assert_eq!(metadata.schema_version, "2.0.0");
        assert!(metadata.items_completed);
        assert!(metadata.places_completed);
        assert_eq!(metadata.stats.item_count, 339);
        assert_eq!(metadata.stats.place_count, 53);
    }

    #[test]
    fn test_load_places_file() {
        // Test loading places starting with '0'
        let places = load_places_file(EXPORT_PATH, '0').expect("Failed to load places file");
        assert!(!places.is_empty());

        // Verify first place has correct structure
        let first_place = &places[0];
        assert!(!first_place.id.is_empty());
        assert!(!first_place.name.is_empty());
    }

    #[test]
    fn test_load_places_file_a() {
        // Test loading places starting with 'A'
        let places = load_places_file(EXPORT_PATH, 'A').expect("Failed to load places file");
        assert!(!places.is_empty());

        // All places should have IDs starting with 'A'
        for place in places {
            assert!(place.id.starts_with('A'));
        }
    }

    #[test]
    fn test_place_cache() {
        let mut cache = PlaceCache::new(EXPORT_PATH);

        // Load a known place ID (from the test data)
        let place = cache
            .get_place("12DF1EED-9AD0-4CB2-87F1-EE8E0FABDFE4")
            .expect("Failed to load place");

        assert_eq!(place.name, "Home");
        assert_eq!(place.visit_count, 70);

        // Verify caching works - should be cached now
        assert!(!cache.is_empty());
        let place2 = cache
            .get_place("12DF1EED-9AD0-4CB2-87F1-EE8E0FABDFE4")
            .expect("Failed to load place from cache");

        // Should be the same Rc
        assert!(Rc::ptr_eq(&place, &place2));
    }

    #[test]
    fn test_load_items_for_month() {
        let items =
            load_items_for_month(EXPORT_PATH, "2025-08").expect("Failed to load items");
        assert!(!items.is_empty());

        // Verify first item structure
        let first_item = &items[0];
        assert!(!first_item.base.id.is_empty());
    }

    #[test]
    fn test_load_all_items() {
        let items = load_all_items(EXPORT_PATH).expect("Failed to load all items");
        assert!(!items.is_empty());

        // Should be loaded in chronological order
        // Verify at least some items exist
        assert!(items.len() > 100);
    }

    #[test]
    fn test_load_items_with_places() {
        let items = load_items_with_places(EXPORT_PATH, "2025-08")
            .expect("Failed to load items with places");

        assert!(!items.is_empty());

        // Find a visit item and verify it has a place
        let visit_item = items.iter().find(|i| i.item.is_visit());
        assert!(visit_item.is_some());

        let visit_item = visit_item.unwrap();
        assert!(visit_item.place.is_some());

        let place = visit_item.place.as_ref().unwrap();
        assert!(!place.name.is_empty());
    }

    #[test]
    fn test_place_sharing_across_items() {
        let items = load_all_items_with_places(EXPORT_PATH)
            .expect("Failed to load all items with places");

        // Find multiple visits to the same place (Home)
        let home_visits: Vec<_> = items
            .iter()
            .filter(|i| {
                i.place
                    .as_ref()
                    .map(|p| p.name == "Home")
                    .unwrap_or(false)
            })
            .collect();

        assert!(home_visits.len() > 1);

        // Verify they all share the same Rc<Place>
        let first_place = home_visits[0].place.as_ref().unwrap();
        for visit in &home_visits[1..] {
            let place = visit.place.as_ref().unwrap();
            assert!(Rc::ptr_eq(first_place, place));
        }
    }
}
