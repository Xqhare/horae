use common::{
    SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE, SECONDS_IN_YEAR, days_in_month,
    is_this_year_leap_year, leap_years_since_epoch, make_now_date, make_now_time, week_day,
};
use date::Date;
use time::Time;

use crate::{
    time_zones::{TimeZone, detect_local_offset},
    tokenizer::{Token, Unit, tokenize},
};

mod common;
pub mod date;
pub mod time;

/// Complete date and time.
/// Also holds the timezone and the unix timestamp.
///
/// Date and time are always in the timezone held.
///
/// Instantiated with `DateTime::now()`, or `DateTime::from_timestamp(timestamp)`.
///
#[derive(Debug, Copy, Clone)]
pub struct DateTime {
    date: Date,
    time: Time,
    pub unix_timestamp: f64,
    pub timezone: f64,
}

impl DateTime {
    /// Instantiates a new `DateTime` with the current date and time.
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
            timezone: 0.0,
        }
    }

    /// Instantiates a new `DateTime` from any supplied unix timestamp.
    pub fn from_timestamp(timestamp: f64) -> DateTime {
        let (date, new_timestamp, unix_timestamp) = make_now_date(timestamp);
        let time = make_now_time(new_timestamp);
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: 0.0,
        }
    }

    /// Returns the held `Time` of the `DateTime`.
    ///
    /// Used for formatting and reading parts of the held `Time`.
    pub fn time(&self) -> Time {
        let (_, time) = self.get_local_components();
        time
    }

    /// Returns the held `Date` of the `DateTime`.
    ///
    /// Used for formatting and reading parts of the held `Date`.
    pub fn date(&self) -> Date {
        let (date, _) = self.get_local_components();
        date
    }

    /// Helper to get local date and time components based on the offset.
    fn get_local_components(&self) -> (Date, Time) {
        if self.timezone == 0.0 {
            return (self.date, self.time);
        }
        let local_timestamp = self.unix_timestamp + self.timezone * SECONDS_IN_HOUR;
        let (date, rest_timestamp, _) = make_now_date(local_timestamp);
        let time = make_now_time(rest_timestamp);
        (date, time)
    }

    /// Returns the formatted string of the `DateTime` according to the supplied formatter.
    ///
    /// Used for formatting the entirety of the `DateTime`.
    pub fn format(&self, formatter: &str) -> String {
        let format_tokens = tokenize(formatter);
        let mut formatted_string = String::new();
        let (local_date, local_time) = self.get_local_components();
        for token in format_tokens {
            match token {
                Token::Unit(unit) => match unit {
                    Unit::Timezone => {
                        if self.timezone == 0.0 {
                            formatted_string.push_str("Coordinated Universal Time");
                        } else {
                            let sign = if self.timezone >= 0.0 { "+" } else { "-" };
                            let abs_offset = self.timezone.abs();
                            let hours = abs_offset.trunc() as i32;
                            let minutes = (abs_offset.fract() * 60.0).round() as i32;
                            formatted_string
                                .push_str(&format!("GMT{sign}{hours:02}:{minutes:02}"));
                        }
                    }
                    Unit::Millisecond => {
                        formatted_string.push_str(&format!("{:03}", local_time.subseconds));
                    }
                    Unit::ShortSecond => {
                        formatted_string.push_str(&format!("{:01}", local_time.second));
                    }
                    Unit::Second => {
                        formatted_string.push_str(&format!("{:02}", local_time.second));
                    }
                    Unit::ShortMinute => {
                        formatted_string.push_str(&format!("{:01}", local_time.minute));
                    }
                    Unit::Minute => {
                        formatted_string.push_str(&format!("{:02}", local_time.minute));
                    }
                    Unit::ShortHour => {
                        formatted_string.push_str(&format!("{:01}", local_time.hour));
                    }
                    Unit::Hour => {
                        formatted_string.push_str(&format!("{:02}", local_time.hour));
                    }
                    Unit::ShortDay => {
                        formatted_string.push_str(&format!("{:01}", local_date.day));
                    }
                    Unit::Day => {
                        formatted_string.push_str(&format!("{:02}", local_date.day));
                    }
                    Unit::ShortNumMonth => {
                        formatted_string.push_str(&format!("{:01}", local_date.month));
                    }
                    Unit::NumMonth => {
                        formatted_string.push_str(&format!("{:02}", local_date.month));
                    }
                    Unit::ShortWordMonth => {
                        const MONTHS: [&str; 12] = [
                            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
                            "Nov", "Dec",
                        ];
                        formatted_string.push_str(MONTHS[local_date.month as usize - 1]);
                    }
                    Unit::WordMonth => {
                        const MONTHS: [&str; 12] = [
                            "January",
                            "February",
                            "March",
                            "April",
                            "May",
                            "June",
                            "July",
                            "August",
                            "September",
                            "October",
                            "November",
                            "December",
                        ];
                        formatted_string.push_str(MONTHS[local_date.month as usize - 1]);
                    }
                    Unit::ShortYear => {
                        formatted_string.push_str(&format!(
                            "{:01}",
                            local_date
                                .year
                                .to_string()
                                .chars()
                                .last()
                                .expect("No Year found!")
                        ));
                    }
                    Unit::Year => {
                        let year_tmp: String =
                            local_date.year.to_string().chars().rev().take(2).collect();
                        let year = year_tmp.chars().rev().collect::<String>();
                        formatted_string.push_str(&year);
                    }
                    Unit::FullYear => {
                        formatted_string.push_str(&format!("{}", local_date.year));
                    }
                    Unit::ShortWeekDay => {
                        let week_day_num =
                            week_day(self.unix_timestamp + self.timezone * SECONDS_IN_HOUR);
                        let week_day = match week_day_num {
                            1 => "Mon",
                            2 => "Tue",
                            3 => "Wed",
                            4 => "Thu",
                            5 => "Fri",
                            6 => "Sat",
                            7 => "Sun",
                            // Should really never happen!
                            _ => "Error",
                        };
                        formatted_string.push_str(week_day);
                    }
                    Unit::WeekDay => {
                        let week_day_num =
                            week_day(self.unix_timestamp + self.timezone * SECONDS_IN_HOUR);
                        let week_day = match week_day_num {
                            1 => "Monday",
                            2 => "Tuesday",
                            3 => "Wednesday",
                            4 => "Thursday",
                            5 => "Friday",
                            6 => "Saturday",
                            7 => "Sunday",
                            // Should really never happen!
                            _ => "Error",
                        };
                        formatted_string.push_str(week_day);
                    }
                    Unit::ShortWeekNumber => {
                        formatted_string.push_str(&format!("{:01}", local_date.get_weeknumber()));
                    }
                    Unit::WeekNumber => {
                        formatted_string.push_str(&format!("{:02}", local_date.get_weeknumber()));
                    }
                },
                Token::Separator(separator) => {
                    formatted_string.push_str(&separator.separator_symbol);
                }
            }
        }

        formatted_string
    }

    /// Returns the week number of the date according to ISO 8601.
    ///
    /// The week number is based on the local date (including timezone).
    pub fn get_weeknumber(&self) -> u8 {
        let (local_date, _) = self.get_local_components();
        local_date.get_weeknumber()
    }

    /// Mutates the `DateTime` to be in the supplied `TimeZone`.
    pub fn with_timezone(&mut self, timezone: TimeZone) {
        self.with_utc_offset(timezone.get_utc_offset());
    }

    /// Mutates the `DateTime` to be in the supplied UTC offset.
    pub fn with_utc_offset(&mut self, offset: f64) {
        self.timezone = offset;
    }

    /// Mutates the `DateTime` to use the system's local timezone.
    pub fn with_auto_offset(&mut self) {
        if let Some(offset) = detect_local_offset() {
            self.with_utc_offset(offset);
        }
    }

    /// Returns the current UTC offset in hours.
    pub fn get_utc_offset(&self) -> f64 {
        self.timezone
    }

    /// Instantiates a new `DateTime` with the specified date and time
    ///
    /// This function assumes that the passed in data is in UTC.
    /// To construct from a local time, use `from_ymd_hms_timezone` or `from_ymd_hms_offset`.
    ///
    /// # Panics
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
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        assert!(hour <= 23);
        assert!(minute <= 59);
        assert!(second <= 59);
        let time = Time::from((hour, minute, second));

        let years = year - 1970;
        let leap_years = {
            let tmp = leap_years_since_epoch(years);
            if month <= 2 && is_this_year_leap_year(year) {
                tmp - 1
            } else {
                tmp
            }
        };
        let years_in_sec = f64::from(years) * SECONDS_IN_YEAR;
        let months_in_sec: f64 = {
            let mut total_days: u16 = 0;
            for i in 1..month {
                total_days += u16::from(days_in_month(i));
            }
            f64::from(total_days) * SECONDS_IN_DAY
        };

        // The calculated date is now on the first of this month.
        // Because of this we need to subtract 1 from day
        let days_in_sec = {
            let total_days = u16::from(day) + leap_years - 1;
            f64::from(total_days) * SECONDS_IN_DAY
        };
        let hours_in_sec = f64::from(hour) * SECONDS_IN_HOUR;
        let minutes_in_sec: f64 = f64::from(minute) * f64::from(SECONDS_IN_MINUTE);
        let unix_timestamp = years_in_sec
            + months_in_sec
            + days_in_sec
            + hours_in_sec
            + minutes_in_sec
            + f64::from(second);
        let date = Date::from((year, month, day, unix_timestamp));
        DateTime {
            date,
            time,
            unix_timestamp,
            timezone: 0.0,
        }
    }

    /// Instantiates a new `DateTime` with the specified date, time and timezone.
    ///
    /// This function assumes that the passed in data is in local time.
    /// To construct from a UTC time, use `from_ymd_hms`.
    ///
    /// # Panics
    /// This function will panic if supplied arguments are out of range for their respective fields
    pub fn from_ymd_hms_timezone(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        timezone: TimeZone,
    ) -> DateTime {
        DateTime::from_ymd_hms_offset(
            year,
            month,
            day,
            hour,
            minute,
            second,
            timezone.get_utc_offset(),
        )
    }

    /// Instantiates a new `DateTime` with the specified date, time and UTC offset.
    ///
    /// This function assumes that the passed in data is in local time.
    /// To construct from a UTC time, use `from_ymd_hms`.
    ///
    /// # Panics
    /// This function will panic if supplied arguments are out of range for their respective fields
    pub fn from_ymd_hms_offset(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset: f64,
    ) -> DateTime {
        let mut out = DateTime::from_ymd_hms(year, month, day, hour, minute, second);
        out.unix_timestamp -= offset * SECONDS_IN_HOUR;
        out.with_utc_offset(offset);
        out
    }
}

// Display implementation

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (date, time) = self.get_local_components();
        write!(f, "{date} {time}")
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
    //
    // Nvm, I fixed it. Apparently...
    let ts_9999_12_31_23_59_59 = DateTime::from_timestamp(253402300799.0);
    assert_eq!(
        "9999-12-31 23:59:59.000".to_string(),
        format!("{}", ts_9999_12_31_23_59_59)
    );
}
