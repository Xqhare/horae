# Horae
Dependency-free, basic time and date rust library.

As a hobby project, I don't think it's ready for production use.

## Motivation
I wrote this library to remove the need for `chrono` or `time` for my time and date handling.
One more library for my tech-stack.

## Roadmap

- Current UTC date and time
- Local date and time in specified timezone
- Basic Date and time arithmetic
    - Add two dates or times together
    - Subtract one date or time from another date or time
    - Add a date or time and a duration
    - Subtract a date or time and a duration

## Features

- Dependency-free

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
let _date_in_past_gmt = Utc.from_ymd(2019, 12, 31).and_hms(23, 59, 59).with_timezone(GMT);
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

// Everything is printable in YYYY-MM-DD HH:MM:SS.MS
println!("{}", now_minus_duration);
// Example: 2019-01-01 09:09:09.000

// Or use format for fine-grained control
println!("{}", now_minus_duration.format("%y-%m-%d %H:%M:%S"));
// Example: 9-1-1 9:9:9
println!("{}", now_minus_duration.format("%yy-%m-%d %H:%M:%S"));
// Example: 19-1-1 9:9:9
println!("{}", now_minus_duration.format("%yyyy-%mm-%dd %HH:%MM:%SS"));
// Example: 2019-01-01 09:09:09
println!("{}", now_minus_duration.format("%yyyy-%mmm-%dd %HH:%MM:%SS"));
// Example: 2019-JAN-01 09:09:09
println!("{}", now_minus_duration.format("%yyyy-%mmmm-%dd %HH:%MM:%SS"));
// Example: 2019-JANUARY-01 09:09:09
println!("{}", now_minus_duration.format("%yyyy/%mm/%dd:%HH-%MM-%SS"));
// Example: 2019/01/01:09-09-09
println!("{}", now_minus_duration.format("%dd-%mm-%yyyy %HH:%MM"));
// Example: 01-01-2019 09:09
println!("{}", now_minus_duration.format("%dd-%mm %HH:%MM"));
// Example: 01-01 09:09
println!("{}", now_minus_duration.format("%mm %MM.%MS"));
// Example: 01 09.000

// Quick note: Upper- and lowercase letters matter for the formatter to work. Lowercase for dates, uppercase for times.

// For only printing the date, use `.date()`. Format is YYYY-MM-DD
println!("{}", now_minus_duration.date());
// Example: 2019-01-01

// Fine-grained control is also possible
println!("{}", now_minus_duration.date().format("%y/%m/%d"));
// Example: 19/1/1

// For only printing the time, use `.time()`. Format is HH:MM:SS.MS
println!("{}", now_minus_duration.time());
// Example: 09:09:09.000

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

