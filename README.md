# Horae
Basic time and date rust library.

As a hobby project, I don't think it's ready for production use.

## Roadmap

- Current UTC date and time
- Local date and time in specified timezone
- Basic Date and time arithmetic
    - Add two dates or times together
    - Subtract one date or time from another date or time
    - Add a date or time and a duration
    - Subtract a date or time and a duration
- Date and time durations
    - Get number of leap years between two dates
    - Get number of years between two dates
    - Get number of months between two dates
    - Get number of weeks between two dates
    - Get number of days between two dates
    - Get number of hours between two dates or times
    - Get number of minutes between two dates or times
    - Get number of seconds between two dates or times
    - Get number of leap seconds between two dates or times
    - Get number of milliseconds between two dates or times

## Creating new dates and times
In general I want the API to feel like this:
```rust

let utc_now = Utc::now();
let _now_in_gmt = Utc::now().with_timezone(GMT);
let _now_in_capital = Utc::now().with_timezone(Timezone::from("Berlin"));
let _now_in_capital2 = Utc::now().with_timezone(Timezone::from("Washington, D.C.")); // D.C. or Washington also work, no state support!
let _now_in_capital3 = Utc::now().with_timezone(Timezone::from("Abu Dhabi"));
let _now_in_capital4 = Utc::now().with_timezone(Timezone::from("Cockburn Town"));

let date_in_past = Utc.from_ymd(2019, 12, 31).and_hms(23, 59, 59);
let date_in_future = Utc.from_ymd(2040, 1, 1).and_hms(0, 0, 0);
let duration = std::time::Duration::from_secs(66_666);

let diff_now_past = utc_now - date_in_past;
let diff_now_future = date_in_future - utc_now; // same as utc_now - date_in_future, future date (larger date) is always set first in subtraction
let now_minus_duration = utc_now - duration;

let now_plus_future = utc_now + date_in_future;
let now_plus_past = date_in_past + utc_now;
let now_plus_duration = utc_now + duration;

let time_now = utc_now.time();

let time_in_past = Time::from_hms(23, 59, 59);
let time_in_future = Time::from_hms(0, 0, 0);

let diff_time_now_past: std::time::Duration = time_now - time_in_past;
let diff_time_now_future: std::time::Duration = time_in_future - time_now;
let time_now_minus_duration = time_now - duration;

let time_now_plus_future: std::time::Duration = time_now + time_in_future;
let time_now_plus_past: std::time::Duration = time_in_past + time_now;
let time_now_plus_duration = time_now + duration;

let leap_years: usize = date_in_future.leap_years_since(date_in_past);
let years: usize = date_in_future.years_since(date_in_past);
let months: usize = date_in_future.months_since(date_in_past);
let weeks: usize = date_in_future.weeks_since(date_in_past);
let days: usize = date_in_future.days_since(date_in_past);
let hours: usize = date_in_future.hours_since(date_in_past);
let minutes: usize = date_in_future.minutes_since(date_in_past);
let seconds: usize = date_in_future.seconds_since(date_in_past);
let leap_seconds: usize = date_in_future.leap_seconds_since(date_in_past);
let milliseconds: usize = date_in_future.milliseconds_since(date_in_past);

// Everything is printable in YYYY-MM-DD HH:MM:SS.MS
println!("{}", now_minus_duration);

// Or use format for fine-grained control
println!("{}", now_minus_duration.format("%y-%m-%d %H:%M:%S"));
// Example: 9-1-1 9:9:9
println!("{}", now_minus_duration.format("%yy-%m-%d %H:%M:%S"));
// Example: 19-1-1 9:9:9
println!("{}", now_minus_duration.format("%yyyy-%mm-%dd %HH:%MM:%SS"));
// Example: 2019-01-01 09:09:09
println!("{}", now_minus_duration.format("%yyyy/%mm/%dd:%HH-%MM-%SS"));
// Example: 2019/01/01:09-09-09
println!("{}", now_minus_duration.format("%dd-%mm-%yyyy %HH:%MM"));
// Example: 01-01-2019 09:09
println!("{}", now_minus_duration.format("%dd-%mm %HH:%MM"));
// Example: 01-01 09:09
println!("{}", now_minus_duration.format("%mm %MM"));
// Example: 01 09

// Quick note: Upper- and lowercase letters matter for the formatter to work. Lowercase for dates, uppercase for times.

// For only printing the date, use `.date()`. Format is YYYY-MM-DD
println!("{}", now_minus_duration.date());
// Example: 2019-01-01

// Fine-grained control is also possible
println!("{}", now_minus_duration.date().format("%y/%m/%d"));
// Example: 19/1/1

// For only printing the time, use `.time()`. Format is HH:MM:SS
println!("{}", now_minus_duration.time());
// Example: 09:09:09

// Fine-grained control is also possible
println!("{}", now_minus_duration.time().format("%H-%M-%S"));
// Example: 9-9-9
```

## Timezone

### Supported Capitals
All supported capitals can be found [here](https://en.wikipedia.org/wiki/List_of_national_capitals).

## Leap Seconds
> WIP Leap seconds are not handled yet.

Leap seconds are calculated based on a table [found here](TODO).

