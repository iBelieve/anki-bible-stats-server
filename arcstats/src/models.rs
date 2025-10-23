use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

/// Metadata about the Arc export
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub samples_completed: bool,
    pub export_mode: String,
    pub session_start_date: f64,
    pub items_completed: bool,
    pub export_type: String,
    pub session_finish_date: f64,
    pub stats: ExportStats,
    pub schema_version: String,
    pub places_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportStats {
    pub sample_count: u32,
    pub item_count: u32,
    pub place_count: u32,
}

/// A place/location from Arc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_mean: f64,
    #[serde(rename = "radiusSD")]
    pub radius_sd: f64,
    pub visit_count: u32,
    pub visit_days: u32,
    pub last_saved: f64,
    pub is_stale: bool,
    pub source: String,
    pub rtree_id: u32,
    pub seconds_from_gmt: Option<i32>,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub country_code: Option<String>,
    pub google_place_id: Option<String>,
    pub google_primary_type: Option<String>,
    pub last_visit_date: Option<f64>,
}

/// Item variant - either a visit or a trip
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemVariant {
    Visit(VisitDetails),
    Trip(TripDetails),
}

/// Timeline item with base data and variant-specific data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub base: BaseItem,
    #[serde(flatten)]
    pub variant: ItemVariant,
}

/// Base fields common to all items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseItem {
    pub id: String,
    pub start_date: f64,
    pub end_date: f64,
    pub last_saved: f64,
    pub source: String,
    pub source_version: Option<String>,
    pub is_visit: bool,
    pub deleted: bool,
    pub disabled: bool,
    pub samples_changed: Option<bool>,
    pub step_count: Option<u32>,
    pub active_energy_burned: Option<f64>,
    pub max_heart_rate: Option<f64>,
    pub average_heart_rate: Option<f64>,
    pub previous_item_id: Option<String>,
    pub next_item_id: Option<String>,
}

/// Details specific to visit items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitDetails {
    pub item_id: String,
    pub place_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_mean: f64,
    #[serde(rename = "radiusSD")]
    pub radius_sd: f64,
    pub confirmed_place: bool,
    pub uncertain_place: bool,
    pub last_saved: f64,
    pub street_address: Option<String>,
}

/// Details specific to trip items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TripDetails {
    pub item_id: String,
    pub distance: f64,
    pub speed: f64,
    pub classified_activity_type: Option<u32>,
    pub confirmed_activity_type: Option<u32>,
    pub uncertain_activity_type: bool,
    pub last_saved: f64,
}

/// Parsed item with resolved place reference
#[derive(Debug, Clone)]
pub struct ItemWithPlace {
    pub item: Item,
    pub place: Option<Rc<Place>>,
}

impl Item {
    /// Check if this item is a visit
    pub fn is_visit(&self) -> bool {
        matches!(self.variant, ItemVariant::Visit(_))
    }

    /// Check if this item is a trip
    pub fn is_trip(&self) -> bool {
        matches!(self.variant, ItemVariant::Trip(_))
    }

    /// Get the place_id if this is a visit
    pub fn place_id(&self) -> Option<&str> {
        match &self.variant {
            ItemVariant::Visit(visit) => visit.place_id.as_deref(),
            ItemVariant::Trip(_) => None,
        }
    }

    /// Get the start date as DateTime
    pub fn start_datetime(&self) -> DateTime<Utc> {
        apple_timestamp_to_datetime(self.base.start_date)
    }

    /// Get the end date as DateTime
    pub fn end_datetime(&self) -> DateTime<Utc> {
        apple_timestamp_to_datetime(self.base.end_date)
    }

    /// Get the duration in seconds
    pub fn duration_seconds(&self) -> f64 {
        self.base.end_date - self.base.start_date
    }
}

impl Place {
    /// Get the last saved date as DateTime
    pub fn last_saved_datetime(&self) -> DateTime<Utc> {
        apple_timestamp_to_datetime(self.last_saved)
    }

    /// Get the last visit date as DateTime if available
    pub fn last_visit_datetime(&self) -> Option<DateTime<Utc>> {
        self.last_visit_date.map(apple_timestamp_to_datetime)
    }
}

/// Convert Apple NSTimeInterval (seconds since 2001-01-01 00:00:00 UTC) to DateTime
pub fn apple_timestamp_to_datetime(timestamp: f64) -> DateTime<Utc> {
    // Apple's reference date is 2001-01-01 00:00:00 UTC
    let apple_epoch = DateTime::parse_from_rfc3339("2001-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    // Add the timestamp seconds to the epoch
    apple_epoch + chrono::Duration::milliseconds((timestamp * 1000.0) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_apple_timestamp_conversion() {
        // Test a known timestamp from the export data
        // session_start_date: 782854313.455665 from metadata.json
        let dt = apple_timestamp_to_datetime(782854313.455665);

        // This should be sometime in 2025
        assert_eq!(dt.year(), 2025);
    }

    #[test]
    fn test_item_helpers() {
        let visit_item = Item {
            base: BaseItem {
                id: "test-id".to_string(),
                start_date: 778085854.759,
                end_date: 778099244.398,
                last_saved: 780692329.0,
                source: "LocoKit".to_string(),
                source_version: Some("9.0.0".to_string()),
                is_visit: true,
                deleted: false,
                disabled: false,
                samples_changed: Some(true),
                step_count: Some(779),
                active_energy_burned: Some(91.36),
                max_heart_rate: Some(121.0),
                average_heart_rate: Some(88.56),
                previous_item_id: None,
                next_item_id: Some("next-id".to_string()),
            },
            variant: ItemVariant::Visit(VisitDetails {
                item_id: "test-id".to_string(),
                place_id: Some("place-id".to_string()),
                latitude: 38.5,
                longitude: -90.4,
                radius_mean: 50.0,
                radius_sd: 10.0,
                confirmed_place: true,
                uncertain_place: false,
                last_saved: 780692328.841,
                street_address: Some("123 Main St".to_string()),
            }),
        };

        assert!(visit_item.is_visit());
        assert!(!visit_item.is_trip());
        assert_eq!(visit_item.place_id(), Some("place-id"));
        assert!(visit_item.duration_seconds() > 0.0);
    }
}
