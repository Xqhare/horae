/*!

# Horae
Dependency-free, basic time and date rust library.

> [!info] As a hobby project, it's not ready for production use.

Horae should never panic. [More here](#panics).
Only dates after January 1, 1970, are supported.

Horae is only as accurate as the UNIX time-stamp provided by the operating system.

## Motivation
I wrote this library to remove the need for `chrono` or `time` for my time and date handling.
It is simply another library for my tech-stack.

## Roadmap

- Fully documented

## Features

- Dependency-free
- Current UTC date and time
    - On average, creating the current date and time takes 100 to 150 nanoseconds.
- Local date and time in specified timezone
    - Supports 200 Timezones
- Basic Date and time arithmetic
    - Add a date and time and a duration
    - Subtract a date and time and a duration
- Custom formatting
    - With weekday option

## Performance

- The only guarantee I am willing to make regarding the maximum creation time is less than 0.5 seconds per instance.
    - In testing, with release optimization, generating 100 million `Utc::now` instances took about 15 seconds.
    - Maximum time observed never exceeded 0.5 seconds for a single instance after testing several hundred million generations.
    - Average times observed were 120, 134, 140, 139 nanoseconds.

## Panics
There is one panic in the library, a system error if UNIX time could not be obtained.

This should almost never happen.

## Usage

### Getting Started
Add Horae to your Cargo.toml as shown in the example below.

```toml
[dependencies]
horae = { git = "https://github.com/Xqhare/horae" }
```

Now run `cargo update` to pull the latest version.

### Instantiation
To create a new date and time, instantiate the `Utc` struct with the `now()` or `from_ymd_hms()` function, depending on your needs.

```rust
use horae::Utc;

let utc_now = Utc::now();
let date_in_past = Utc::from_ymd_hms(2019, 12, 31, 23, 59, 59);
let date_in_future = Utc::from_ymd_hms(2040, 1, 1, 0, 0, 0);
```

### Adding a Timezone
In most use-cases it is desirable to add a timezone to a date and time.
To do so, use the `with_timezone()` function.

The `with_timezone()` takes a specific `TimeZone` enum as an argument.
[See the `TimeZone` documentation for more information](#timezone).

The date and time held accessible inside the `Utc` struct is always in the supplied timezone.

```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::now();
let mut now_in_CEST = Utc::now();
now_in_CEST.with_timezone(TimeZone::CentralEuropeanSummerTime);
```

### Arithmetic
Basic date and time arithmetic can be done with the `Utc` struct and a `Duration` from the standard library.

> [!note] Adding or subtracting a `Utc` from another `Utc` is not supported.

```rust
use horae::Utc;

let utc_now = Utc::now();
let utc_plus_day = utc_now + std::time::Duration::from_secs(86_400);
let utc_minus_day = utc_now - std::time::Duration::from_secs(86_400);
```

### Formatting
By default Horae formats the date and time as `YYYY-MM-DD HH:MM:SS.MS`.
The `Utc` struct also provides the `date()` and `time()` functions ([explained here](#date-and-time)) to print only the date or time respectively.

```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);

println!("{}", utc_now);
// Example: 2019-01-01 09:09:09.000

assert_eq!("2019-01-01 09:09:09.000", utc_now.to_string());

assert_eq!("2019-01-01", utc_now.date().to_string());

assert_eq!("09:09:09.000", utc_now.time().to_string());

```

#### Custom Formatting
Horae provides a `format()` function to format the date and time any way you want.

```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);

println!("{}", utc_now.format("%yyyy/%mm/%dd::%HH-%MM-%SS,%MS"));
// Example: 2019/01/01:09-09-09,000

assert_eq!("2019/01/01::09-09-09,000", utc_now.format("%yyyy/%mm/%dd::%HH-%MM-%SS,%MS").to_string());

assert_eq!("2019/01/01", utc_now.date().format("%yyyy/%mm/%dd").to_string());

assert_eq!("09-09-09,000", utc_now.time().format("%HH-%MM-%SS,%MS").to_string());
```

The same effect as using `.date()` or `.time()` can be achieved with the `format()` function on the `Utc` struct.
```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);

assert_eq!("2019/01/01", utc_now.format("%yyyy/%mm/%dd").to_string());

assert_eq!("09-09-09,000", utc_now.format("%HH-%MM-%SS,%MS").to_string());
```

## API
The complete functionality of Horae is shown in the example below.

```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::now();
let mut now_in_gmt = Utc::now();
now_in_gmt.with_timezone(TimeZone::GreenwichMeanTime);

let date_in_past = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
let date_in_past_gmt = Utc::from_ymd_hms_timezone(2019, 12, 31, 23, 59, 59, TimeZone::GreenwichMeanTime);
let date_in_future = Utc::from_ymd_hms(2040, 1, 1, 0, 0, 0);
let duration = std::time::Duration::from_secs(66_666);

let now_minus_duration = utc_now - duration;
let now_plus_duration = utc_now + duration;

// Everything is printable in YYYY-MM-DD HH:MM:SS.MS
println!("{}", date_in_past);
assert_eq!("2019-01-01 09:09:09.000", date_in_past.to_string());

// Or use format for fine-grained control
assert_eq!("9-1-1 9:9:9", date_in_past.format("%y-%m-%d %H:%M:%S").to_string());
assert_eq!("19-1-1 9:9:9", date_in_past.format("%yy-%m-%d %H:%M:%S").to_string());
assert_eq!("2019-01-01 09:09:09", date_in_past.format("%yyyy-%mm-%dd %HH:%MM:%SS").to_string());

assert_eq!("2019-Jan-01 09:09:09", date_in_past.format("%yyyy-%mmm-%dd %HH:%MM:%SS").to_string());
assert_eq!("2019-January-01 09:09:09.000", date_in_past.format("%yyyy-%mmmm-%dd %HH:%MM:%SS.%MS").to_string());

assert_eq!("2019/01/01:09-09-09", date_in_past.format("%yyyy/%mm/%dd:%HH-%MM-%SS").to_string());
assert_eq!("01-01-2019 09:09", date_in_past.format("%dd-%mm-%yyyy %HH:%MM").to_string());
assert_eq!("01-01 09:09", date_in_past.format("%dd-%mm %HH:%MM").to_string());
assert_eq!("01 09.000", date_in_past.format("%mm %MM.%MS").to_string());

assert_eq!("Tue", date_in_past.format("%wd").to_string());
assert_eq!("Tuesday", date_in_past.format("%wdd").to_string());

// Quick note: Upper- and lowercase letters matter for the formatter to work. Lowercase for dates, uppercase for times.

// For only printing the date, use `.date()`. Format is YYYY-MM-DD
assert_eq!("2019-01-01", date_in_past.date().to_string());

// Fine-grained control is also possible
assert_eq!("9/1/1", date_in_past.date().format("%y/%m/%d").to_string());

// For only printing the time, use `.time()`. Format is HH:MM:SS.MS
assert_eq!("09:09:09.000", date_in_past.time().to_string());

// Fine-grained control is also possible
assert_eq!("9-9-9", date_in_past.time().format("%H-%M-%S").to_string());
```

## Date and Time
Horae provides both a representation of date and time for further use.

The date and time are representations are converted into the supplied timezone.
If no timezone is supplied, UTC is used and the date and time is in UTC.

```rust
use horae::{Utc, TimeZone};

let utc_now = Utc::now();
let mut cest_now = Utc::now();
cest_now.with_timezone(TimeZone::CentralEuropeanSummerTime);

assert_ne!(utc_now.time().hour, cest_now.time().hour);
```

### Date
The `date()` function returns a `Date` struct.

It holds the year, month, and day of the date.

```rust
use horae::Utc;

let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);

let date = utc_now.date();

println!("{}", date);

assert_eq!("2019-01-01", date.to_string());
assert_eq!(2019, date.year);
assert_eq!(1, date.month);
assert_eq!(1, date.day);
```

### Time
The `time()` function returns a `Time` struct.

It holds the hour, minute, second, and millisecond of the time.

```rust
use horae::Utc;

let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);

let time = utc_now.time();

println!("{}", time);

assert_eq!("09:09:09.000", time.to_string());
assert_eq!(9, time.hour);
assert_eq!(9, time.minute);
assert_eq!(9, time.second);
assert_eq!(0, time.subseconds);
```

## Timezone
Horae supports about 200 timezones.

All supported timezones can be found [here](https://en.wikipedia.org/wiki/List_of_time_zone_abbreviations).
The list is up-to-date as of 2024-10-20;

### Supported Timezones
The `TimeZone` enum has the `get_all()` function to get a list of all supported timezones.

```rust
use horae::TimeZone;

let timezones = TimeZone::get_all();
for timezone in timezones {
    println!("{}", timezone);
}
```

## Leap Seconds
While Horae has some functions to support leap seconds, they are not used in the library.

The simple reason: It does not matter for the calculations from UNIX time to a human-readable date.

This does make this library not as accurate as it could be and unsuitable for production use.
The inaccuracy is only a problem for dates during a leap second.

This would also be the only part of Horae that would need constant maintenance, as leap seconds are added arbitrarily.

This feature would only be needed to ensure that supplied dates are encoded into UNIX time correctly, never to decode them. I think.

*/
use std::time::Duration;

use date_time::{date::Date, time::Time, DateTime};

mod date_time;
mod time_zones;
mod tokenizer;

pub use crate::time_zones::TimeZone;

#[derive(Debug, Clone, Copy)]
pub struct Utc {
    date_time: DateTime,
}

impl Utc {
    pub fn now() -> Utc {
        Utc {
            date_time: DateTime::now(),
        }
    }

    pub fn with_timezone(&mut self, timezone: TimeZone) {
        self.date_time.with_timezone(timezone);
    }

    pub fn from_ymd_hms(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms(year, month, day, hour, minute, second),
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
    ) -> Utc {
        Utc {
            date_time: DateTime::from_ymd_hms_timezone(
                year, month, day, hour, minute, second, timezone,
            ),
        }
    }

    pub fn time(&self) -> Time {
        self.date_time.time()
    }

    pub fn date(&self) -> Date {
        self.date_time.date()
    }

    pub fn format(&self, formatter: &str) -> String {
        self.date_time.format(formatter)
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
