use std::fmt::Display;

use chrono::{DateTime, Utc};

use crate::types::FDateTime;

const NANOS_PER_TICK: i64 = 100;
const TICKS_PER_SECOND: i64 = 10_000_000;
const UNIX_OFFSET_SECS: i64 = 62_135_596_800;
const UNIX_OFFSET_TICKS: i64 = UNIX_OFFSET_SECS * TICKS_PER_SECOND;

impl FDateTime {
    pub fn from_date_time(dt: DateTime<Utc>) -> Option<Self> {
        let ticks = dt
            .timestamp_nanos_opt()?
            .checked_div(NANOS_PER_TICK)?
            .checked_add(UNIX_OFFSET_TICKS)?;
        Some(Self { ticks })
    }

    pub fn to_datetime(&self) -> Option<DateTime<Utc>> {
        let unix_nanos = self
            .ticks
            .checked_sub(UNIX_OFFSET_TICKS)?
            .checked_mul(NANOS_PER_TICK)?;
        Some(DateTime::from_timestamp_nanos(unix_nanos))
    }

    pub fn now() -> Self {
        Utc::now().into()
    }

    pub fn format_datetime(&self) -> String {
        match self.to_datetime() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S%.f UTC").to_string(),
            None => format!("pre-epoch date (ticks={})", self.ticks),
        }
    }
}

impl Display for FDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format_datetime())
    }
}

impl From<DateTime<Utc>> for FDateTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::from_date_time(dt)
            .unwrap_or_else(|| unimplemented!("Unable to create FDateTime from {dt}"))
    }
}
