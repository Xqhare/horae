use common::{
    days_in_month, is_this_year_leap_year, leap_years_since_epoch, make_now_date, make_now_time,
    SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE, SECONDS_IN_YEAR,
};
use date::Date;
use time::Time;

use crate::time_zones::TimeZone;

mod common;
mod date;
mod time;

#[derive(Debug, Copy, Clone)]
pub struct DateTime {
    date: Date,
    time: Time,
    pub unix_timestamp: f64,
    pub timezone: TimeZone,
}

impl DateTime {
    pub fn now() -> DateTime {
        // Only logically unguarded panic in the library below!
        let (date, timestamp, unix_timestamp) = {
            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
            match timestamp {
                Ok(duration) => make_now_date(duration.as_secs_f64()),
                Err(e) => panic!("{}", e),
            }
        };
        let time = make_now_time(timestamp);
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: TimeZone::Utc,
        }
    }

    pub fn from_timestamp(timestamp: f64) -> DateTime {
        let (date, new_timestamp, unix_timestamp) = make_now_date(timestamp);
        let time = make_now_time(new_timestamp);
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
            let hours = tmp.trunc() as i16;
            let minutes: i8 = {
                match tmp.fract() {
                    // These are the only fractions to exist - For now I am sure
                    f if f == 0.0 => 0,
                    f if f == 0.25 => 15,
                    f if f == 0.5 => 30,
                    f if f == 0.75 => 45,
                    f if f == -0.25 => -15,
                    f if f == -0.5 => -30,
                    f if f == -0.75 => -45,
                    _ => 0,
                }
            };
            (hours, minutes)
        };
        self.set_timezone(timezone);

        // First calculate new time, if hour is negative, go back 1 day
        if utc_offset_hours.is_positive() {
            let tmp_minute_bind = self.time.minute + utc_offset_minutes as u8;
            // Again while bigger than 60, will never be bigger than 105 or so
            if tmp_minute_bind > 59 {
                self.time.hour += 1;
                // calc rest minutes of hour
                let rest_minutes = tmp_minute_bind - 60;
                debug_assert!(rest_minutes < 60);
                // add difference to next hour minutes
                self.time.minute = rest_minutes;
            } else {
                self.time.minute = tmp_minute_bind;
            }

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
                let rest_hours = tmp_hour_bind - self.time.hour as i16;
                debug_assert!(rest_hours < 24);
                // expect: Ok, because previous logic ensures:
                // rest_hours < 24 and 24 is smaller than 255
                self.time.hour = TryInto::<u8>::try_into(rest_hours).expect("Everything checked!");
            } else {
                // Same day
                // expect: Ok, because previous logic ensures:
                // rest_hours < 24 and 24 is smaller than 255
                self.time.hour =
                    TryInto::<u8>::try_into(tmp_hour_bind).expect("Everything checked!");
            }
        } else {
            // utc_offset_minutes is positive, but needs to be subtracted
            let tmp_minute_bind = {
                if self.time.minute == 0 {
                    utc_offset_minutes * -1
                } else {
                    self.time.minute as i8 + utc_offset_minutes
                }
            };
            // Again while smaller than 0, will never be smaller than -45 or so
            if tmp_minute_bind.is_negative() {
                self.time.hour -= 1;
                // calc rest minutes of hour
                let rest_minutes = 60 + tmp_minute_bind;
                debug_assert!(rest_minutes < 60);
                // add difference to next hour minutes
                self.time.minute = rest_minutes as u8;
            } else {
                debug_assert!(tmp_minute_bind < 60);
                self.time.minute = tmp_minute_bind as u8;
            }

            // utc_offset_hours is negative!
            let tmp_hour_bind = self.time.hour as i16 + utc_offset_hours;
            // While possibly smaller than 0, will never be smaller than -12 or so.
            // - 12h to UTC
            if tmp_hour_bind.is_negative() {
                // previous day && possibly previous month && possibly previous year
                if self.date.day - 1 == 0 {
                    // previous month && possibly previous year
                    if self.date.month - 1 == 0 {
                        // previous year
                        self.date.year -= 1;
                        self.date.month = 12;
                    } else {
                        self.date.month -= 1;
                    }
                    self.date.day = days_in_month(self.date.month);
                } else {
                    self.date.day -= 1;
                }
                let actual_rest_hours = 24 + tmp_hour_bind;
                // expect: Ok, because previous logic ensures:
                // tmp_hour_bind > -24 and 24 + -24 is smaller than 255 and positive
                self.time.hour =
                    TryInto::<u8>::try_into(actual_rest_hours).expect("Everything checked!");
            } else {
                // Same day
                // expect: Ok, because previous logic ensures:
                // tmp_hour_bind is positive and thus >= 0 and smaller 24
                self.time.hour =
                    TryInto::<u8>::try_into(tmp_hour_bind).expect("Everything checked!");
            }
        }
    }

    fn set_timezone(&mut self, timezone: TimeZone) {
        self.timezone = timezone;
    }

    /// This function will panic if supplied arguments are out of range for their respective fields
    pub fn from_ymd_hms(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> DateTime {
        assert!(year >= 1970);
        assert!(month >= 1 && month <= 12);
        assert!(day >= 1 && day <= 31);
        assert!(hour <= 23);
        assert!(minute <= 59);
        assert!(second <= 59);
        let date = Date::from_ymd(year, month, day);
        let time = Time::from_hms(hour, minute, second);

        let years = year - 1970;
        let leap_years = {
            let tmp = leap_years_since_epoch(years);
            if month <= 2 && is_this_year_leap_year(year) {
                tmp - 1
            } else {
                tmp
            }
        };
        let years_in_sec = years as f64 * SECONDS_IN_YEAR;
        let months_in_sec: f64 = {
            let mut total_days: u16 = 0;
            for i in 1..month {
                total_days += days_in_month(i) as u16;
            }
            total_days as f64 * SECONDS_IN_DAY
        };

        // The calculated date is now on the first of this month.
        // Because of this we need to subtract 1 from day
        let days_in_sec = {
            let total_days = day as u16 + leap_years - 1;
            total_days as f64 * SECONDS_IN_DAY
        };
        let hours_in_sec = hour as f64 * SECONDS_IN_HOUR;
        let minutes_in_sec: f64 = minute as f64 * SECONDS_IN_MINUTE as f64;
        let unix_timestamp = years_in_sec
            + months_in_sec
            + days_in_sec
            + hours_in_sec
            + minutes_in_sec
            + second as f64;
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: TimeZone::Utc,
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
    ) -> DateTime {
        let mut out = DateTime::from_ymd_hms(year, month, day, hour, minute, second);
        out.with_timezone(timezone);
        out
    }
}

// Display implementation

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

#[test]
fn from_timestamp() {
    let ts_1970_01_01_00_00_00 = DateTime::from_timestamp(0.0);
    assert_eq!(
        "1970-01-01 00:00:00.000".to_string(),
        format!("{}", ts_1970_01_01_00_00_00)
    );
    let ts_1999_04_21_10_02_45 = DateTime::from_timestamp(924688965.0);
    assert_eq!(
        "1999-04-21 10:02:45.000".to_string(),
        format!("{}", ts_1999_04_21_10_02_45)
    );
    let ts_1976_01_01_00_00_00 = DateTime::from_timestamp(189298800.0);
    assert_eq!(
        "1975-12-31 23:00:00.000".to_string(),
        format!("{}", ts_1976_01_01_00_00_00)
    );
    let ts_1970_01_14_02_03_10 = DateTime::from_timestamp(1130590.958881855);
    assert_eq!(
        "1970-01-14 02:03:10.958881855".to_string(),
        format!("{}", ts_1970_01_14_02_03_10)
    );
    let ts_2054_06_10_08_36_47 = DateTime::from_timestamp(2664686207.0);
    assert_eq!(
        "2054-06-10 06:36:47.000".to_string(),
        format!("{}", ts_2054_06_10_08_36_47)
    );
    let ts_5997_01_15_04_27_14 = DateTime::from_timestamp(32410297634.0);
    assert_eq!(
        "2997-01-15 04:27:14.000".to_string(),
        format!("{}", ts_5997_01_15_04_27_14)
    );
    let ts_9876_05_22_16_56_43 = DateTime::from_timestamp(249501574603.0);
    assert_eq!(
        "9876-05-22 16:56:43.000".to_string(),
        format!("{}", ts_9876_05_22_16_56_43)
    );
    // largest timestamp I could generate
    // After long hours of troubleshooting I found that this timestamp is too large and does
    // not work on any of the websites I tried.
    // Some website would not generate a timestamp with this date, others would generate
    // this answer. And not decode it correctly if fed back to the website.
    let ts_9999_12_31_23_59_59 = DateTime::from_timestamp(253402300799.0);
    assert_ne!(
        "9999-12-31 23:59:59.000".to_string(),
        format!("{}", ts_9999_12_31_23_59_59)
    );
}
