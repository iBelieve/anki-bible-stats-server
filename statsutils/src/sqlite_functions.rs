use anyhow::{Context, Result};
use chrono::{Datelike, Duration, TimeZone};
use chrono_tz::Tz;
use rusqlite::Connection;

use crate::config;

/// Converts timestamp to date string with timezone and rollover applied
fn timestamp_ms_to_date_string(timestamp_ms: i64) -> Result<String> {
    let tz: Tz = config::TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    // Convert timestamp to datetime in Chicago timezone
    let dt = tz
        .timestamp_millis_opt(timestamp_ms)
        .single()
        .context("Failed to convert timestamp to datetime")?;

    // Subtract rollover hours to get the logical date
    let adjusted_dt = dt - Duration::hours(config::ROLLOVER_HOUR);

    // Format as YYYY-MM-DD
    Ok(adjusted_dt.format("%Y-%m-%d").to_string())
}

/// Converts timestamp to week string (Sunday of that week)
fn timestamp_ms_to_week_string(timestamp_ms: i64) -> Result<String> {
    let tz: Tz = config::TIMEZONE
        .parse()
        .context("Failed to parse timezone from config")?;

    // Convert timestamp to datetime in Chicago timezone
    let dt = tz
        .timestamp_millis_opt(timestamp_ms)
        .single()
        .context("Failed to convert timestamp to datetime")?;

    // Subtract rollover hours to get the logical datetime
    let adjusted_dt = dt - Duration::hours(config::ROLLOVER_HOUR);

    // Find the Sunday of this week
    let days_since_sunday = adjusted_dt.weekday().num_days_from_sunday();
    let sunday = adjusted_dt - Duration::days(days_since_sunday as i64);

    // Format as YYYY-MM-DD
    Ok(sunday.format("%Y-%m-%d").to_string())
}

/// Registers custom SQLite functions: date_str_from_ms/sec, week_str_from_ms/sec
///
/// All functions apply timezone and 4 AM rollover
pub fn register_date_functions(conn: &Connection) -> Result<()> {
    // date_str_from_ms: milliseconds -> YYYY-MM-DD
    conn.create_scalar_function(
        "date_str_from_ms",
        1,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let timestamp_ms = ctx.get::<i64>(0)?;
            timestamp_ms_to_date_string(timestamp_ms)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))
        },
    )
    .context("Failed to register date_str_from_ms function")?;

    // date_str_from_sec: seconds -> YYYY-MM-DD
    conn.create_scalar_function(
        "date_str_from_sec",
        1,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let timestamp_sec = ctx.get::<i64>(0)?;
            let timestamp_ms = timestamp_sec * 1000;
            timestamp_ms_to_date_string(timestamp_ms)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))
        },
    )
    .context("Failed to register date_str_from_sec function")?;

    // week_str_from_ms: milliseconds -> Sunday YYYY-MM-DD
    conn.create_scalar_function(
        "week_str_from_ms",
        1,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let timestamp_ms = ctx.get::<i64>(0)?;
            timestamp_ms_to_week_string(timestamp_ms)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))
        },
    )
    .context("Failed to register week_str_from_ms function")?;

    // week_str_from_sec: seconds -> Sunday YYYY-MM-DD
    conn.create_scalar_function(
        "week_str_from_sec",
        1,
        rusqlite::functions::FunctionFlags::SQLITE_UTF8
            | rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let timestamp_sec = ctx.get::<i64>(0)?;
            let timestamp_ms = timestamp_sec * 1000;
            timestamp_ms_to_week_string(timestamp_ms)
                .map_err(|e| rusqlite::Error::UserFunctionError(e.into()))
        },
    )
    .context("Failed to register week_str_from_sec function")?;

    Ok(())
}
