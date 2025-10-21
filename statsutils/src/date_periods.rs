use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, TimeZone};
use chrono_tz::Tz;
use std::collections::HashMap;

use crate::config;

/// Time period with date strings and millisecond boundaries
#[derive(Debug, Clone)]
pub struct DatePeriod {
    /// Date strings (YYYY-MM-DD) for each day/week in the period
    pub dates: Vec<String>,
    /// Start timestamp in milliseconds
    pub start_ms: i64,
    /// End timestamp in milliseconds
    pub end_ms: i64,
}

impl DatePeriod {
    /// Returns DatePeriod for the last 30 days
    pub fn last_30_days() -> Result<Self> {
        let (start_ms, _, _) = get_day_boundaries(29)?;
        let (_, end_ms, _) = get_day_boundaries(0)?;

        let mut dates = Vec::new();
        for day_offset in (0..30).rev() {
            let (_, _, date_str) = get_day_boundaries(day_offset)?;
            dates.push(date_str);
        }

        Ok(DatePeriod {
            dates,
            start_ms,
            end_ms,
        })
    }

    /// Returns DatePeriod for the last 12 weeks (Sunday to Sunday)
    pub fn last_12_weeks() -> Result<Self> {
        let (start_ms, _, _) = get_week_boundaries(11)?;
        let (_, end_ms, _) = get_week_boundaries(0)?;

        let mut dates = Vec::new();
        for week_offset in (0..12).rev() {
            let (_, _, week_start_str) = get_week_boundaries(week_offset)?;
            dates.push(week_start_str);
        }

        Ok(DatePeriod {
            dates,
            start_ms,
            end_ms,
        })
    }

    /// Builds results for all dates, using defaults for missing entries
    pub fn build_results<T, R>(
        self,
        mut results: HashMap<String, T>,
        mut mapper: impl FnMut(String, T) -> R,
    ) -> Vec<R>
    where
        T: Default,
    {
        self.dates
            .into_iter()
            .map(|date| {
                let value = results.remove(&date).unwrap_or_else(|| T::default());
                mapper(date, value)
            })
            .collect()
    }

    /// Builds results for all dates from two sources, using defaults for missing entries
    pub fn build_results_2<T1, T2, R>(
        self,
        mut results1: HashMap<String, T1>,
        mut results2: HashMap<String, T2>,
        mut mapper: impl FnMut(String, T1, T2) -> R,
    ) -> Vec<R>
    where
        T1: Default,
        T2: Default,
    {
        self.dates
            .into_iter()
            .map(|date| {
                let value1 = results1.remove(&date).unwrap_or_else(|| T1::default());
                let value2 = results2.remove(&date).unwrap_or_else(|| T2::default());
                mapper(date, value1, value2)
            })
            .collect()
    }
}

/// Returns the start of today in milliseconds (applies 4 AM rollover)
pub fn get_today_start_ms() -> Result<i64> {
    let (today_start_ms, _, _) = get_day_boundaries(0)?;
    Ok(today_start_ms)
}

/// Calculates day boundaries with 4 AM rollover
/// Returns (start_ms, end_ms, date_str)
fn get_day_boundaries(day_offset: i32) -> Result<(i64, i64, String)> {
    let tz: Tz = config::TIMEZONE
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
    let day_start = day_midnight + Duration::hours(config::ROLLOVER_HOUR);

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

/// Calculates week boundaries (Sunday start, 4 AM rollover)
/// Returns (start_ms, end_ms, week_start_str)
fn get_week_boundaries(week_offset: i32) -> Result<(i64, i64, String)> {
    let tz: Tz = config::TIMEZONE
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
    let week_start = week_midnight + Duration::hours(config::ROLLOVER_HOUR);

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
