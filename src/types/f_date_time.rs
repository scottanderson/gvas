use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};

use crate::types::FDateTime;

const NANOS_PER_TICK: i64 = 100;
const TICKS_PER_SECOND: i64 = 10_000_000;
const UNIX_OFFSET_SECS: i64 = 62_135_596_800;
const UNIX_OFFSET_TICKS: i64 = UNIX_OFFSET_SECS * TICKS_PER_SECOND;

#[derive(Debug, thiserror::Error)]
pub enum FDateTimeError {
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("Time is out of range")]
    OutOfRange,
}

impl FDateTime {
    pub const fn from_date_time(dt: DateTime<Utc>) -> Result<Self, FDateTimeError> {
        // Option::ok_or is not yet const-stable, issue 143956.
        let Some(ts_nanos) = dt.timestamp_nanos_opt() else {
            return Err(FDateTimeError::OutOfRange);
        };
        // This division cannot overflow
        let ts_ticks = ts_nanos / NANOS_PER_TICK;
        let Some(ticks) = ts_ticks.checked_add(UNIX_OFFSET_TICKS) else {
            return Err(FDateTimeError::OutOfRange);
        };
        Ok(Self { ticks })
    }

    pub const fn to_date_time(&self) -> Result<DateTime<Utc>, FDateTimeError> {
        // Option::ok_or is not yet const-stable, issue 143956.
        let Some(ts_ticks) = self.ticks.checked_sub(UNIX_OFFSET_TICKS) else {
            return Err(FDateTimeError::OutOfRange);
        };
        let Some(ts_nanos) = ts_ticks.checked_mul(NANOS_PER_TICK) else {
            return Err(FDateTimeError::OutOfRange);
        };
        Ok(DateTime::from_timestamp_nanos(ts_nanos))
    }

    pub fn now() -> Result<Self, FDateTimeError> {
        Utc::now().try_into()
    }

    pub fn format_datetime(&self) -> String {
        match self.to_date_time() {
            Ok(dt) => dt.format("%Y-%m-%d %H:%M:%S%.f UTC").to_string(),
            Err(_) => format!("pre-epoch date (ticks={})", self.ticks),
        }
    }
}

impl Display for FDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format_datetime())
    }
}

impl FromStr for FDateTime {
    type Err = FDateTimeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_time = chrono::DateTime::from_str(s)?;
        Self::from_date_time(date_time)
    }
}

impl TryFrom<DateTime<Utc>> for FDateTime {
    type Error = FDateTimeError;

    #[inline]
    fn try_from(dt: DateTime<Utc>) -> Result<Self, Self::Error> {
        Self::from_date_time(dt)
    }
}
