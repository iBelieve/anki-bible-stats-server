use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use chrono_tz::America::Chicago;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::loader::load_all_items_with_places;
use statsutils::DatePeriod;

/// Weekly statistics for church attendance
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WeekStats {
    /// Week start date in YYYY-MM-DD format (Sunday)
    pub week_start: String,
    /// Time spent at church in minutes
    pub minutes: f64,
}

/// Statistics for a single place showing time spent
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PlaceStats {
    /// Name of the place
    pub place_name: String,
    /// Total hours spent at this place
    pub hours: f64,
}

/// Converts a UTC datetime to a week start date string (YYYY-MM-DD)
/// Applies 4 AM rollover and finds the most recent Sunday in Chicago timezone
fn get_week_start_for_datetime(dt: DateTime<Utc>) -> String {
    const ROLLOVER_HOUR: i64 = 4;

    // Convert to Chicago timezone
    let dt_chicago = dt.with_timezone(&Chicago);

    // Apply 4 AM rollover: if before 4 AM, consider it part of previous day
    let adjusted_dt = if dt_chicago.hour() < ROLLOVER_HOUR as u32 {
        dt_chicago - Duration::hours(24)
    } else {
        dt_chicago
    };

    // Calculate days since last Sunday (0 if today is Sunday)
    let days_since_sunday = adjusted_dt.weekday().num_days_from_sunday();

    // Go back to the most recent Sunday
    let week_start = adjusted_dt - Duration::days(days_since_sunday as i64);

    // Format as YYYY-MM-DD
    week_start.format("%Y-%m-%d").to_string()
}

/// Gets church attendance statistics for the last 12 weeks
///
/// # Arguments
///
/// * `export_path` - Path to the Arc Timeline export directory containing places/, items/, and metadata.json
///
/// # Returns
///
/// A vector of 12 WeekStats, one for each week, in chronological order.
/// Weeks without church visits will have 0 minutes.
pub fn get_last_12_weeks_stats(export_path: &str) -> Result<Vec<WeekStats>> {
    // Get the period data for the last 12 weeks
    let period = DatePeriod::last_12_weeks()?;

    // Load all items with their associated places
    let items = load_all_items_with_places(export_path)?;

    // Filter for visits at places containing "church" (case-insensitive)
    // and calculate duration in minutes for each visit
    let mut church_visits: Vec<(DateTime<Utc>, f64)> = Vec::new();

    for item_with_place in items {
        // Skip if not a visit
        if !item_with_place.item.base.is_visit {
            continue;
        }

        // Skip if no place or place name doesn't contain "church"
        if let Some(place) = &item_with_place.place {
            if place.name.to_lowercase().contains("church") {
                let start = item_with_place.item.start_datetime();
                let duration_minutes = item_with_place.item.duration_seconds() / 60.0;
                church_visits.push((start, duration_minutes));
            }
        }
    }

    // Group visits by week and sum minutes
    let mut weekly_minutes: HashMap<String, f64> = HashMap::new();

    for (visit_time, minutes) in church_visits {
        let week_start = get_week_start_for_datetime(visit_time);
        *weekly_minutes.entry(week_start).or_insert(0.0) += minutes;
    }

    // Build results for all 12 weeks, filling gaps with 0 minutes
    let results = period.build_results(weekly_minutes, |date, minutes| WeekStats {
        week_start: date,
        minutes,
    });

    Ok(results)
}

/// Gets the top N places by total hours spent over the last 6 months
///
/// # Arguments
///
/// * `export_path` - Path to the Arc Timeline export directory containing places/, items/, and metadata.json
/// * `limit` - Maximum number of places to return (e.g., 10 for top 10)
///
/// # Returns
///
/// A vector of PlaceStats sorted by hours descending (most time first).
/// Excludes the place named "Home".
pub fn get_top_places_last_6_months(export_path: &str, limit: usize) -> Result<Vec<PlaceStats>> {
    const DAYS_IN_6_MONTHS: i64 = 182;

    // Calculate the cutoff date (6 months ago)
    let now = Utc::now();
    let cutoff_date = now - Duration::days(DAYS_IN_6_MONTHS);

    // Load all items with their associated places
    let items = load_all_items_with_places(export_path)?;

    // Collect visits with place names and durations
    let mut place_durations: HashMap<String, f64> = HashMap::new();

    for item_with_place in items {
        // Skip if not a visit
        if !item_with_place.item.base.is_visit {
            continue;
        }

        // Skip if no place
        let Some(place) = &item_with_place.place else {
            continue;
        };

        // Skip if place name is "Home"
        if place.name == "Home" {
            continue;
        }

        // Skip if visit is before cutoff date
        let visit_start = item_with_place.item.start_datetime();
        if visit_start < cutoff_date {
            continue;
        }

        // Calculate duration in hours
        let duration_hours = item_with_place.item.duration_seconds() / 3600.0;

        // Add to place total
        *place_durations.entry(place.name.clone()).or_insert(0.0) += duration_hours;
    }

    // Convert to vec of PlaceStats and sort by hours descending
    let mut place_stats: Vec<PlaceStats> = place_durations
        .into_iter()
        .map(|(place_name, hours)| PlaceStats { place_name, hours })
        .collect();

    place_stats.sort_by(|a, b| {
        b.hours
            .partial_cmp(&a.hours)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Take top N
    place_stats.truncate(limit);

    Ok(place_stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_week_stats_structure() {
        let stats = WeekStats {
            week_start: "2025-10-19".to_string(),
            minutes: 120.5,
        };

        assert_eq!(stats.week_start, "2025-10-19");
        assert_eq!(stats.minutes, 120.5);
    }
}
