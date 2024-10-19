use common::{make_now_date, make_now_time};
use date::Date;
use time::Time;

use crate::time_zones::TimeZone;

mod date;
mod time;
mod common;



pub struct DateTime {
    date: Date,
    time: Time,
    unix_timestamp: f64,
    timezone: TimeZone,
}

impl DateTime {
    pub fn now() -> DateTime {
        let (date, timestamp, unix_timestamp) = make_now_date();
        let time = make_now_time(timestamp);
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: TimeZone::UTC,
        }
    }
    
    pub fn with_timezone(timezone: TimeZone) -> DateTime {
        let mut out = DateTime::now();
        out.set_timezone(timezone);
        out
    }

    fn set_timezone(&mut self, timezone: TimeZone) {
        self.timezone = timezone;
    }
}

// Display implementation

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
