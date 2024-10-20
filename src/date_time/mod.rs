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
        let (date, timestamp, unix_timestamp) = make_now_date(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
        let time = make_now_time(timestamp);
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: TimeZone::Utc,
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

    // TODO: add unix_timestamp calculation
    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
        let date = Date::from_ymd(year, month, day);
        let time = Time::from_hms(hour, minute, second);
        DateTime {
            date,
            time,
            unix_timestamp: 0.0,
            timezone: TimeZone::Utc,
        }
    }

    // TODO: add unix_timestamp calculation
    pub fn from_ymd_hms_timezone(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8, timezone: TimeZone) -> DateTime {
        let date = Date::from_ymd(year, month, day);
        let time = Time::from_hms(hour, minute, second);
        DateTime {
            date,
            time,
            unix_timestamp: 0.0,
            timezone,
        }
    }
}

// Display implementation

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
