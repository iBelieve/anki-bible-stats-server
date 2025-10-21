use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, TimeZone};
use chrono_tz::Tz;

/// Timezone used for calculating day boundaries across all stats crates
pub const TIMEZONE: &str = "America/Chicago";

/// Rollover hour for determining day boundaries (4 AM)
/// Days start at 4 AM instead of midnight to better reflect human activity patterns
pub const ROLLOVER_HOUR: i64 = 4;

/// Helper function to get the start of today in epoch milliseconds
///
/// Uses the standard 4 AM rollover hour defined in ROLLOVER_HOUR.
///
/// # Returns
/// Unix timestamp in milliseconds for the start of today
pub fn get_today_start_ms() -> Result<i64> {
    let (today_start_ms, _, _) = get_day_boundaries(0)?;
    Ok(today_start_ms)
}

/// Helper function to calculate day boundaries for a specific day offset
///
/// Uses the standard 4 AM rollover hour defined in ROLLOVER_HOUR.
///
/// # Arguments
/// * `day_offset` - Number of days before today (0 = today, 1 = yesterday, etc.)
///
/// # Returns
/// A tuple of (day_start_ms, day_end_ms, date_str) where:
/// - day_start_ms: Unix timestamp in milliseconds for start of day
/// - day_end_ms: Unix timestamp in milliseconds for end of day (start of next day)
/// - date_str: Date in YYYY-MM-DD format
pub fn get_day_boundaries(day_offset: i32) -> Result<(i64, i64, String)> {
    let tz: Tz = TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    let now_in_tz = Local::now().with_timezone(&tz);

    // Calculate the target date (today - day_offset)
    let target_date = now_in_tz - Duration::days(day_offset as i64);

    // Get start of that day at midnight
    let day_midnight = tz
        .with_ymd_and_hms(
            target_date.year(),
            target_date.month(),
            target_date.day(),
            0,
            0,
            0,
        )
        .single()
        .context("Failed to create target day's midnight")?;

    // Add rollover hours for day start
    let day_start = day_midnight + Duration::hours(ROLLOVER_HOUR);

    // Next day start
    let next_day_start = day_start + Duration::days(1);

    // Format date for output
    let date_str = target_date.format("%Y-%m-%d").to_string();

    Ok((
        day_start.timestamp_millis(),
        next_day_start.timestamp_millis(),
        date_str,
    ))
}

/// Helper function to calculate week boundaries for a specific week offset
///
/// Weeks start on Sunday and use the standard 4 AM rollover hour defined in ROLLOVER_HOUR.
///
/// # Arguments
/// * `week_offset` - Number of weeks before current week (0 = current week, 1 = last week, etc.)
///
/// # Returns
/// A tuple of (week_start_ms, week_end_ms, week_start_str) where:
/// - week_start_ms: Unix timestamp in milliseconds for start of week (Sunday)
/// - week_end_ms: Unix timestamp in milliseconds for end of week (next Sunday)
/// - week_start_str: Date of week start (Sunday) in YYYY-MM-DD format
pub fn get_week_boundaries(week_offset: i32) -> Result<(i64, i64, String)> {
    let tz: Tz = TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    let now_in_tz = Local::now().with_timezone(&tz);

    // Calculate days since last Sunday (0 if today is Sunday)
    let days_since_sunday = now_in_tz.weekday().num_days_from_sunday();

    // Calculate the target Sunday (go back to most recent Sunday, then subtract week_offset weeks)
    let target_date =
        now_in_tz - Duration::days(days_since_sunday as i64) - Duration::weeks(week_offset as i64);

    // Get start of that Sunday at midnight
    let week_midnight = tz
        .with_ymd_and_hms(
            target_date.year(),
            target_date.month(),
            target_date.day(),
            0,
            0,
            0,
        )
        .single()
        .context("Failed to create week's midnight")?;

    // Add rollover hours for week start
    let week_start = week_midnight + Duration::hours(ROLLOVER_HOUR);

    // Next week start (7 days later)
    let next_week_start = week_start + Duration::weeks(1);

    // Format week start date for output
    let week_start_str = target_date.format("%Y-%m-%d").to_string();

    Ok((
        week_start.timestamp_millis(),
        next_week_start.timestamp_millis(),
        week_start_str,
    ))
}
