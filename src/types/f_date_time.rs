use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};

use crate::{error::FDateTimeError, types::FDateTime};

const NANOS_PER_TICK: i64 = 100;
const TICKS_PER_SECOND: i64 = 10_000_000;
const UNIX_OFFSET_SECS: i64 = 62_135_596_800;
const UNIX_OFFSET_TICKS: i64 = UNIX_OFFSET_SECS * TICKS_PER_SECOND;

const RAW_TICKS_PREFIX: &str = "pre-epoch date (ticks=";
const RAW_TICKS_SUFFIX: &str = ")";

impl FDateTime {
    #[cfg(test)]
    pub(crate) const fn from_ticks(ticks: i64) -> Self {
        Self { ticks }
    }

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
            Err(_) => format!("{}{}{}", RAW_TICKS_PREFIX, self.ticks, RAW_TICKS_SUFFIX),
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
        if let Some(ticks) = s
            .strip_prefix(RAW_TICKS_PREFIX)
            .and_then(|s| s.strip_suffix(RAW_TICKS_SUFFIX))
        {
            return ticks
                .parse()
                .map(|ticks| Self { ticks })
                .map_err(|_| FDateTimeError::OutOfRange);
        }
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

#[cfg(test)]
mod test {
    use crate::error::Result;

    use super::*;

    const BEFORE_EPOCH_DATE_TIME: FDateTime = FDateTime::from_ticks(0);
    const BEFORE_EPOCH_STR: &str = "pre-epoch date (ticks=0)";

    const VALID_DATE_TIME: FDateTime = FDateTime::from_ticks(637864237380020000);
    const VALID_STR: &str = "2022-04-24 19:02:18.002 UTC";

    #[test]
    fn str() -> Result<()> {
        for (dt, s) in [
            (BEFORE_EPOCH_DATE_TIME, BEFORE_EPOCH_STR),
            (VALID_DATE_TIME, VALID_STR),
        ] {
            let fs = FDateTime::from_str(s)?;
            let ts = dt.format_datetime();
            assert_eq!(dt, fs);
            assert_eq!(s, ts);
        }
        Ok(())
    }
}
