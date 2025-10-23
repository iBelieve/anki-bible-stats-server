//! Arc Timeline Export Parser
//!
//! This library provides models and loaders for parsing Arc Timeline app exports.
//! Arc exports location timeline data including visits to places and trips between them.
//!
//! # Usage
//!
//! ```no_run
//! use arcstats::{load_metadata, load_all_items_with_places};
//!
//! let export_path = "path/to/arc/export";
//!
//! // Load metadata
//! let metadata = load_metadata(export_path).unwrap();
//! println!("Export contains {} items", metadata.stats.item_count);
//!
//! // Load all items with their associated places
//! let items = load_all_items_with_places(export_path).unwrap();
//! for item in items {
//!     if let Some(place) = &item.place {
//!         println!("Visit to {} at {}", place.name, item.item.start_datetime());
//!     }
//! }
//! ```

pub mod loader;
pub mod models;
pub mod stats;

// Re-export commonly used types and functions
pub use loader::{
    load_all_items, load_all_items_with_places, load_all_places, load_items_for_month,
    load_items_with_places, load_metadata, load_places_file, PlaceCache,
};
pub use models::{
    apple_timestamp_to_datetime, BaseItem, ExportStats, Item, ItemWithPlace, Metadata, Place,
    TripDetails, VisitDetails,
};
pub use stats::{get_last_12_weeks_stats, WeekStats};
