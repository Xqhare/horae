use common::{days_in_month, make_now_date, make_now_time};
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
    
    pub fn with_timezone(&mut self, timezone: TimeZone) {
        let (utc_offset_hours, utc_offset_minutes) = {
            let tmp = timezone.get_utc_offset();
            let hours = tmp.floor() as i16;
            let minutes: u8 = {
                match tmp.fract() {
                    // These are the only fractions to exist - For now I am sure
                    f if f == 0.0 => 0,
                    f if f == 0.25 => 15,
                    f if f == 0.5 => 30,
                    f if f == 0.75 => 45,
                    _ => 0,
                }
            };
            (hours, minutes)
        };
        self.set_timezone(timezone);

        // First calculate new time, if hour is negative, go back 1 day
        // TODO what about years?
        if utc_offset_hours.is_positive() {
            let tmp_hour_bind = self.time.hour as i16 + utc_offset_hours;
            // While possibly bigger than 24, will never be bigger than 36. 
            // + 12h to UTC
            if tmp_hour_bind > 23 {
                // next day && possibly next month && possibly next year
                if self.date.day + 1 > days_in_month(self.date.month) {
                    // next month && possibly next year
                    if self.date.month + 1 > 12 {
                        // next year
                        self.date.year += 1;
                        self.date.month = 1;
                    } else {
                        self.date.month += 1;
                    }
                    self.date.day = 1;
                } else {
                    self.date.day += 1;
                }
                let rest_hours = tmp_hour_bind - (24 - self.time.hour as i16);
                debug_assert!(rest_hours < 24);
                self.time.hour = TryInto::<u8>::try_into(rest_hours).expect("Everything checked!");
            } else {
                // Same day
                self.time.hour = TryInto::<u8>::try_into(tmp_hour_bind).expect("Everything checked!");
            }

            let tmp_minute_bind = self.time.minute + utc_offset_minutes;
            // Again while bigger than 60, will never be bigger than 105 or so
            if tmp_minute_bind > 59 {
                self.time.hour += 1;
                // calc rest minutes of hour
                let rest_minutes = tmp_minute_bind - (60 - self.time.minute);
                debug_assert!(rest_minutes < 60);
                // add difference to next hour minutes
                self.time.minute = rest_minutes;
            } else {
                self.time.minute = tmp_minute_bind;
            }
        // TODO
        } else {
            
        }
        
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
