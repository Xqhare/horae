use date::Date;
use time::Time;

use crate::time_zones::TimeZone;

mod date;
mod time;

const SECONDS_IN_DAY: f64 = 86_400.0;
const DAYS_IN_YEAR_APPROX: f64 = 365.0;

pub struct DateTime {
    date: Date,
    time: Time,
    unix_timestamp: f64,
    timezone: TimeZone,
}

impl DateTime {
    pub fn now() -> DateTime {
        let unix_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        let days_since_epoch = unix_timestamp / SECONDS_IN_DAY;
        let years_since_epoch = (days_since_epoch / DAYS_IN_YEAR_APPROX).floor();
        let leap_years
        DateTime {
            date: Date::now(),
            time: Time::now(),
            unix_timestamp,
            timezone: TimeZone::UTC,
        }
    }
    
}
