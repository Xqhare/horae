use date_time::DateTime;
use time_zones::TimeZone;

mod date_time;
mod time_zones;

pub struct Utc {
    date_time: DateTime,
}

impl Utc {
    pub fn now() -> Utc {
        Utc {
            date_time: DateTime::now(),
        }
    }

    pub fn with_timezone(timezone: TimeZone) -> Utc {
        Utc {
            date_time: DateTime::with_timezone(timezone),
        }
    }

    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms(year, month, day, hour, minute, second),
        }
    }
}

impl std::fmt::Display for Utc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.date_time)
    }
}
