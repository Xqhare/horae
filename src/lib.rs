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
}
