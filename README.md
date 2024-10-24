# Horae
Dependency-free, basic time and date rust library.

> ![info] As a hobby project, I don't think it's ready for production use.

Horae should never panic. [More here](#panics).
Only dates after January 1, 1970, are supported.

Horae is only as accurate as the UNIX time-stamp provided by the operating system.

## Motivation
I wrote this library to remove the need for `chrono` or `time` for my time and date handling.
One more library for my tech-stack.

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

## Performance

- The only guarantee I am willing to make regarding the maximum creation time is less than 0.5 seconds per instance.
    - In testing, with release optimization, generating 100 million `Utc::now` instances took about 15 seconds.
    - Maximum time observed never exceeded 0.5 seconds for a single instance after testing several hundred million generations.
    - Average times observed were 120, 134, 140, 139 nanoseconds.

## Panics
There is one panic in the library, a system error if UNIX time could not be obtained.

This should almost never happen.

## Creating new dates and times
In general I want the API to feel like this:
```rust
use horae::{TimeZone, Utc};

let utc_now = Utc::now();
let _now_in_gmt = Utc::now().with_timezone(TimeZone::GreenwichMeanTime);

let date_in_past = Utc.from_ymd_hms(2019, 12, 31, 23, 59, 59);
let _date_in_past_gmt = Utc.from_ymd_hms_timezone(2019, 12, 31, 23, 59, 59, TimeZone::GreenwichMeanTime);
let date_in_future = Utc.from_ymd_hms(2040, 1, 1, 0, 0, 0);
let duration = std::time::Duration::from_secs(66_666);

let now_minus_duration = utc_now - duration;
let now_plus_duration = utc_now + duration;

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
println!("{}", now_minus_duration.format("%yyyy-%mmmm-%dd %HH:%MM:%SS.%MS"));
// Example: 2019-JANUARY-01 09:09:09.000
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

### Supported Timezones
All supported timezones can be found [here](https://en.wikipedia.org/wiki/List_of_time_zone_abbreviations).

## Leap Seconds
> WIP Leap seconds are not handled yet.

Leap seconds are calculated based on a table [found here](TODO).

