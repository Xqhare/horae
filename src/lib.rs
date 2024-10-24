use std::time::Duration;

use date_time::{date::Date, time::Time, DateTime};

mod date_time;
mod time_zones;
mod tokenizer;

pub use crate::time_zones::TimeZone;

#[derive(Debug, Clone, Copy)]
pub struct Utc {
    date_time: DateTime,
}

impl Utc {
    pub fn now() -> Utc {
        Utc {
            date_time: DateTime::now(),
        }
    }

    pub fn with_timezone(&mut self, timezone: TimeZone) {
        self.date_time.with_timezone(timezone);
    }

    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms(year, month, day, hour, minute, second),
        }
    }

    pub fn from_ymd_hms_timezone(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        timezone: TimeZone,
    ) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms_timezone(
                year, month, day, hour, minute, second, timezone,
            ),
        }
    }

    pub fn time(&self) -> Time {
        self.date_time.time()
    }

    pub fn date(&self) -> Date {
        self.date_time.date()
    }

    pub fn format(&self, formatter: &str) -> String {
        self.date_time.format(formatter)
    }
}

// Add duration implementation

impl std::ops::Add<Duration> for Utc {
    type Output = Utc;

    fn add(self, rhs: Duration) -> Utc {
        let new_timestamp = self.date_time.unix_timestamp + rhs.as_secs_f64();
        let mut date_time = DateTime::from_timestamp(new_timestamp);
        date_time.with_timezone(self.date_time.timezone);
        Utc { date_time }
    }
}

// Sub duration implementation

impl std::ops::Sub<Duration> for Utc {
    type Output = Utc;

    fn sub(self, rhs: Duration) -> Utc {
        let new_timestamp = self.date_time.unix_timestamp - rhs.as_secs_f64();
        let mut date_time = DateTime::from_timestamp(new_timestamp);
        date_time.with_timezone(self.date_time.timezone);
        Utc { date_time }
    }
}

// Display implementation

impl std::fmt::Display for Utc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.date_time)
    }
}
