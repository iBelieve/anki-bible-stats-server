/// Timezone used for calculating day boundaries across all stats crates
pub const TIMEZONE: &str = "America/Chicago";

/// Rollover hour for determining day boundaries (4 AM)
/// Days start at 4 AM instead of midnight to better reflect human activity patterns
pub const ROLLOVER_HOUR: i64 = 4;
