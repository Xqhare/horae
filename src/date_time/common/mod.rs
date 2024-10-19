use crate::date_time::date::Date;

use super::time::Time;

const SECONDS_IN_MINUTE: u8 = 60;
const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_DAY: f64 = 86_400.0;
const SECONDS_IN_YEAR: f64 = 31_536_000.0;
const DAYS_IN_YEAR_APPROX: f64 = 365.0;
const EPOCH_YEAR: u16 = 1970;
const NUMBER_OF_DAYS_PER_MONTH: [u8; 12] = [
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
];
// All leap seconds from 1972-2024 
// tuple.0 = year
// tuple.1 = (30. June / 31. Dec)
const LEAP_SECONDS_ARRAY: [(u16, (bool, bool)); 53] = [
    (1972, (true, true)),
    (1973, (false, true)),
    (1974, (false, true)),
    (1975, (false, true)),
    (1976, (false, true)),
    (1977, (false, true)),
    (1978, (false, true)),
    (1979, (false, true)),
    (1980, (false, false)),
    (1981, (true, false)),
    (1982, (true, false)),
    (1983, (true, false)),
    (1984, (false, false)),
    (1985, (true, false)),
    (1986, (false, false)),
    (1987, (false, true)),
    (1988, (false, false)),
    (1989, (false, true)),
    (1990, (false, true)),
    (1991, (false, false)),
    (1992, (true, false)),
    (1993, (true, false)),
    (1994, (true, false)),
    (1995, (false, true)),
    (1996, (false, false)),
    (1997, (true, false)),
    (1998, (false, true)),
    (1999, (false, false)),
    (2000, (false, false)),
    (2001, (false, false)),
    (2002, (false, false)),
    (2003, (false, false)),
    (2004, (false, false)),
    (2005, (false, true)),
    (2006, (false, false)),
    (2007, (false, false)),
    (2008, (false, true)),
    (2009, (false, false)),
    (2010, (false, false)),
    (2011, (false, false)),
    (2012, (true, false)),
    (2013, (false, false)),
    (2014, (false, false)),
    (2015, (true, false)),
    (2016, (false, true)),
    (2017, (false, false)),
    (2018, (false, false)),
    (2019, (false, false)),
    (2020, (false, false)),
    (2021, (false, false)),
    (2022, (false, false)),
    (2023, (false, false)),
    (2024, (false, false)),
];

pub fn leap_years_since_epoch(years_since_epoch: u16) -> u16 {
    let mut leap_years = 0;
    for year in 1970..=(1970 + years_since_epoch) {
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    leap_years += 1;
                }
            } else {
                leap_years += 1;
            }
        }
    }
    leap_years
}

/// Unix time does not count leap seconds -> add them to the number of seconds
///
/// I am accurate to the second without it...
/// Im gonna leave it it because it was a lot of works
#[allow(dead_code)]
pub fn leap_seconds_since_epoch(years_since_epoch: u16) -> u16 {
    let mut leap_seconds = 0;
    for tuple in LEAP_SECONDS_ARRAY {
        if years_since_epoch >= tuple.0 {
            if tuple.1.0 {
                leap_seconds += 1;
            }
            if tuple.1.1 {
                leap_seconds += 1;
            }
        }
    }
    leap_seconds
}

pub fn make_now_time(rest_timestamp: f64) -> Time {
    let mut rest_timestamp = rest_timestamp;
    let hour = (rest_timestamp / SECONDS_IN_HOUR ).floor() as u8;
    rest_timestamp -= hour as f64 * SECONDS_IN_HOUR;
    let minute = (rest_timestamp / SECONDS_IN_MINUTE as f64).floor() as u8;
    rest_timestamp -= minute as f64 * SECONDS_IN_MINUTE as f64;
    let second = rest_timestamp.floor() as u8;
    let rest = rest_timestamp - second as f64;
    Time::from_hmsns(hour, minute, second, rest)
}

pub fn make_now_date(timestamp: f64) -> (Date, f64, f64) {
    let mut tmp_timestamp = timestamp.clone();

    let years_since_epoch = ((timestamp / SECONDS_IN_DAY).floor() / DAYS_IN_YEAR_APPROX).floor();
    let leap_years = leap_years_since_epoch(years_since_epoch as u16);
    let year = EPOCH_YEAR + years_since_epoch as u16;
    tmp_timestamp -= years_since_epoch * SECONDS_IN_YEAR;
    // Somehow the above logic is off by 2 days. I have searched, I have calculated. I dont know
    // why. I truly am sorry.
    tmp_timestamp += 2.0 * SECONDS_IN_DAY;

    let days_this_year = (tmp_timestamp / SECONDS_IN_DAY).floor() - leap_years as f64;
    // remove leap years form tmp_timestamp
    tmp_timestamp -= leap_years as f64 * SECONDS_IN_DAY;
    tmp_timestamp -= days_this_year as f64 * SECONDS_IN_DAY;

    let mut month: u8 = 0;
    let mut days_into_the_year: u16 = 0;
    while (days_into_the_year as f64) < days_this_year {
        days_into_the_year += NUMBER_OF_DAYS_PER_MONTH[month as usize] as u16;
        month += 1;
    }
    let completed_months = month.saturating_sub(1);
    let completed_month_days = {
        let mut out = 1;
        for i in 0..completed_months {
            out += NUMBER_OF_DAYS_PER_MONTH[i as usize] as u16;
        }
        out
    };
    //let completed_month_days = NUMBER_OF_DAYS_PER_MONTH.iter().take(completed_months as usize).map(|x| *x as u16).sum::<u16>();
    debug_assert!(days_into_the_year >= completed_month_days as u16);
    let days_left_in_month = days_this_year as u16 - completed_month_days;

    let day: u8 = {
        debug_assert!(days_left_in_month >= 1);
        debug_assert!(days_left_in_month <= 31);
        days_left_in_month.try_into().expect("Could not convert.")
    };
    
    // now at most 24 hours are left
    debug_assert!(tmp_timestamp <= SECONDS_IN_DAY);
    let date = Date::from_ymd(year, month, day);
    (date, tmp_timestamp, timestamp)
}
