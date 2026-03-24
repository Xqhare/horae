// include README as doc
#![doc = include_str!("../README.md")]

use std::time::Duration;

use date_time::{DateTime, date::Date, time::Time};

mod date_time;
mod time_zones;
mod tokenizer;

pub use crate::time_zones::TimeZone;
pub use crate::time_zones::detect_local_offset as detect_local_utc_offset;

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
    #[must_use] 
    pub fn now() -> Utc {
        Utc {
            date_time: DateTime::now(),
        }
    }

    /// Mutates a `Utc` with the specified timezone enum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use horae::{Utc, TimeZone};
    ///
    /// let mut cest_now = Utc::now();
    /// cest_now.with_timezone(TimeZone::CentralEuropeanSummerTime);
    /// println!("CEST: {}", cest_now);
    /// ```
    pub fn with_timezone<T: Into<TimeZone>>(&mut self, timezone: T) {
        self.date_time.with_timezone(timezone.into());
    }

    /// Mutates a `Utc` with the specified UTC offset in hours.
    ///
    /// # Example
    ///
    /// ```rust
    /// use horae::Utc;
    ///
    /// let mut custom_now = Utc::now();
    /// custom_now.with_utc_offset(5.5); // IST
    /// println!("IST: {}", custom_now);
    /// ```
    pub fn with_utc_offset(&mut self, offset: f64) {
        self.date_time.with_utc_offset(offset);
    }

    /// Mutates a `Utc` by automatically detecting the system's local timezone.
    ///
    /// # Example
    ///
    /// ```rust
    /// use horae::Utc;
    ///
    /// let mut local_now = Utc::now();
    /// local_now.with_auto_offset();
    /// println!("Local: {}", local_now);
    /// ```
    pub fn with_auto_offset(&mut self) {
        self.date_time.with_auto_offset();
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
    #[must_use] 
    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms(year, month, day, hour, minute, second),
        }
    }

    /// Instantiates a new `Utc` with the specified date, time and timezone.
    ///
    /// The date and time are assumed to be local to the specified timezone.
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
    /// let local = Utc::from_ymd_hms_timezone(2019, 1, 1, 9, 9, 9, TimeZone::CentralEuropeanSummerTime);
    /// println!("{}", local);
    /// assert_eq!(local.to_string(), "2019-01-01 09:09:09.000");
    /// // CEST is UTC+2, so the underlying UTC time is 07:09:09
    /// assert_eq!(local.unix_timestamp(), Utc::from_ymd_hms(2019, 1, 1, 7, 9, 9).unix_timestamp());
    /// ```
    #[must_use] 
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

    /// Instantiates a new `Utc` with the specified date, time and UTC offset in hours.
    ///
    /// The date and time are assumed to be local to the specified offset.
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
    /// let local = Utc::from_ymd_hms_offset(2019, 1, 1, 9, 9, 9, 5.5); // IST
    /// println!("{}", local);
    /// assert_eq!(local.to_string(), "2019-01-01 09:09:09.000");
    /// ```
    #[must_use] 
    pub fn from_ymd_hms_offset(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset: f64,
    ) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms_offset(
                year, month, day, hour, minute, second, offset,
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
    #[must_use] 
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
    #[must_use] 
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
    #[must_use] 
    pub fn format(&self, formatter: &str) -> String {
        self.date_time.format(formatter)
    }

    /// Returns the current UTC offset in hours.
    ///
    /// # Examples
    /// ```rust
    /// use horae::{Utc, TimeZone};
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.get_utc_offset(), 0.0);
    /// ```
    #[must_use] 
    pub fn get_utc_offset(&self) -> f64 {
        self.date_time.get_utc_offset()
    }

    /// Returns the week number of the date according to ISO 8601.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2026, 3, 12, 9, 9, 9);
    /// assert_eq!(utc_now.get_weeknumber(), 11);
    /// ```
    #[must_use] 
    pub fn get_weeknumber(&self) -> u8 {
        self.date_time.get_weeknumber()
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
    #[must_use] 
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
    #[must_use] 
    pub fn from_timestamp(timestamp: f64) -> Utc {
        let mut date_time = DateTime::from_timestamp(timestamp);
        date_time.with_utc_offset(0.0);
        Utc { date_time }
    }
}

impl From<f64> for Utc {
    fn from(timestamp: f64) -> Self {
        Utc::from_timestamp(timestamp)
    }
}

impl From<Utc> for f64 {
    fn from(utc: Utc) -> Self {
        utc.unix_timestamp()
    }
}

impl From<std::time::SystemTime> for Utc {
    fn from(st: std::time::SystemTime) -> Self {
        match st.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => Utc::from_timestamp(d.as_secs_f64()),
            Err(_) => Utc::from_timestamp(0.0),
        }
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
        date_time.with_utc_offset(self.date_time.timezone);
        Utc { date_time }
    }
}

// Sub duration implementation

impl std::ops::Sub<Duration> for Utc {
    type Output = Utc;

    fn sub(self, rhs: Duration) -> Utc {
        let new_timestamp = self.date_time.unix_timestamp - rhs.as_secs_f64();
        let mut date_time = DateTime::from_timestamp(new_timestamp);
        date_time.with_utc_offset(self.date_time.timezone);
        Utc { date_time }
    }
}

// Display implementation

impl std::fmt::Display for Utc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.date_time)
    }
}
