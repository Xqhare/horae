// include README as doc
#![doc = include_str!("../README.md")]

use std::time::Duration;

use date_time::{date::Date, time::Time, DateTime};

mod date_time;
mod time_zones;
mod tokenizer;

pub use crate::time_zones::TimeZone;

#[derive(Debug, Clone, Copy)]
/// Basic building block for date and time.
/// Instantiate with `Utc::now()`, or `Utc::from_ymd_hms(year, month, day, hour, minute, second)`.
/// Mutate with `with_timezone` to set a timezone.
///
/// # Examples
/// ```rust
/// use horae::{Utc, TimeZone};
///
/// let utc_now = Utc::now();
/// println!("Current UTC: {}", utc_now);
/// ```
///
/// ```rust
/// use horae::{Utc, TimeZone};
///
/// let mut utc_now = Utc::now();
/// utc_now.with_timezone(TimeZone::CentralEuropeanSummerTime);
/// println!("Current UTC in CEST: {}", utc_now);
/// ```
///
/// ```rust
/// use horae::{Utc, TimeZone};
///
/// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
/// println!("Custom UTC: {}", utc_now);
/// ```
///
/// ```rust
/// use horae::{Utc, TimeZone};
///
/// let mut utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
/// utc_now.with_timezone(TimeZone::CentralEuropeanSummerTime);
/// println!("Custom UTC in CEST: {}", utc_now);
/// ```
pub struct Utc {
    date_time: DateTime,
}

impl Utc {
    /// Instantiates a new `Utc` with the current date and time.
    ///
    /// # Example
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::now();
    /// println!("{}", utc_now);
    /// assert_ne!(utc_now.to_string(), "1970-01-01 00:00:00.000");
    /// ```
    pub fn now() -> Utc {
        Utc {
            date_time: DateTime::now(),
        }
    }

    /// Mutates a `Utc` with the specified timezone.
    ///
    /// # Example
    ///
    /// ```rust
    /// use horae::{Utc, TimeZone};
    ///
    /// let utc_now = Utc::now();
    /// let mut cest_now = Utc::now();
    /// cest_now.with_timezone(TimeZone::CentralEuropeanSummerTime);
    /// println!("UTC: {}", utc_now);
    /// println!("CEST: {}", cest_now);
    /// assert_ne!(utc_now.to_string(), cest_now.to_string());
    /// ```
    pub fn with_timezone<T: Into<TimeZone>>(&mut self, timezone: T) {
        self.date_time.with_timezone(timezone.into());
    }

    /// Instantiates a new `Utc` with the specified date and time.
    ///
    /// # Panics
    /// Will panic if the date is invalid.
    /// Valid ranges are:
    /// - year: 1970-9999
    /// - month: 1-12
    /// - day: 1-31
    /// - hour: 0-23
    /// - minute: 0-59
    /// - second: 0-59
    ///
    /// # Example
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// println!("{}", utc_now);
    /// assert_eq!(utc_now.to_string(), "2019-01-01 09:09:09.000");
    /// ```
    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms(year, month, day, hour, minute, second),
        }
    }

    /// Instantiates a new `Utc` with the specified date, time and timezone.
    ///
    /// # Panics
    /// Will panic if the date is invalid.
    /// Valid ranges are:
    /// - year: 1970-9999
    /// - month: 1-12
    /// - day: 1-31
    /// - hour: 0-23
    /// - minute: 0-59
    /// - second: 0-59
    ///
    /// # Example
    /// ```rust
    /// use horae::{Utc, TimeZone};
    ///
    /// let utc_now = Utc::from_ymd_hms_timezone(2019, 1, 1, 9, 9, 9, TimeZone::CentralEuropeanSummerTime);
    /// println!("{}", utc_now);
    /// assert_eq!(utc_now.to_string(), "2019-01-01 11:09:09.000");
    /// ```
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

    /// Returns the current `Time` of the `Utc` instance.
    ///
    /// Useful for formatting or reading parts of `Time`.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.time().format("%HH:%MM:%SS"), "09:09:09");
    /// assert_eq!(utc_now.time().hour, 9);
    /// assert_eq!(utc_now.time().minute, 9);
    /// assert_eq!(utc_now.time().second, 9);
    /// assert_eq!(utc_now.time().subseconds, 0);
    /// ```
    pub fn time(&self) -> Time {
        self.date_time.time()
    }

    /// Returns the current `Date` of the `Utc` instance.
    ///
    /// Useful for formatting or reading parts of `Date`.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.date().format("%yyyy-%mm-%dd"), "2019-01-01");
    /// assert_eq!(utc_now.date().year, 2019);
    /// assert_eq!(utc_now.date().month, 1);
    /// assert_eq!(utc_now.date().day, 1);
    /// ```
    pub fn date(&self) -> Date {
        self.date_time.date()
    }

    /// Returns the formatted string of the `Utc` instance according to the supplied formatter.
    ///
    /// Useful for formatting the entirety of the `Utc` instance.
    ///
    /// For more information on the available formatting syntax, see the README in the API chapter.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.format("%yyyy-%mm-%dd %HH:%MM:%SS | %tz"), "2019-01-01 09:09:09 | Coordinated Universal Time");
    /// ```
    pub fn format(&self, formatter: &str) -> String {
        self.date_time.format(formatter)
    }

    /// Returns the `TimeZone` of the `Utc` instance.
    ///
    /// # Examples
    /// ```rust
    /// use horae::{Utc, TimeZone};
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.timezone(), TimeZone::CoordinatedUniversalTime);
    /// ```
    pub fn timezone(&self) -> TimeZone {
        self.date_time.timezone
    }

    /// Returns the unix timestamp of the `Utc` instance.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.unix_timestamp(), 1546333749.0);
    /// ```
    pub fn unix_timestamp(&self) -> f64 {
        self.date_time.unix_timestamp
    }

    /// Instantiates a new `Utc` from a unix timestamp.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_timestamp(1546333749.0);
    /// assert_eq!(utc_now.to_string(), "2019-01-01 09:09:09.000");
    /// ```
    pub fn from_timestamp(timestamp: f64) -> Utc {
        let mut date_time = DateTime::from_timestamp(timestamp);
        date_time.with_timezone(TimeZone::CoordinatedUniversalTime);
        Utc { date_time }
    }
}

impl std::ops::Sub<Utc> for Utc {
    type Output = Duration;

    fn sub(self, rhs: Utc) -> Duration {
        Duration::from_secs_f64(self.unix_timestamp() - rhs.unix_timestamp())
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
