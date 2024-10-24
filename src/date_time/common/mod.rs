use crate::date_time::date::Date;

use super::time::Time;

pub const SECONDS_IN_MINUTE: u8 = 60;
pub const SECONDS_IN_HOUR: f64 = 3600.0;
pub const SECONDS_IN_DAY: f64 = 86_400.0;
pub const SECONDS_IN_YEAR: f64 = 31_536_000.0;
const DAYS_IN_YEAR_APPROX: f64 = 365.0;
const EPOCH_YEAR: u16 = 1970;
const NUMBER_OF_DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
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

/// Takes the actual month number (January is 1)
/// and returns the number of days in that month
pub fn days_in_month(month: u8) -> u8 {
    NUMBER_OF_DAYS_PER_MONTH[month as usize - 1]
}

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

pub fn is_this_year_leap_year(year: u16) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            }
        } else {
            return true;
        }
    }
    false
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
            if tuple.1 .0 {
                leap_seconds += 1;
            }
            if tuple.1 .1 {
                leap_seconds += 1;
            }
        }
    }
    leap_seconds
}

pub fn make_now_time(rest_timestamp: f64) -> Time {
    let mut rest_timestamp = rest_timestamp;
    let hour = (rest_timestamp / SECONDS_IN_HOUR).floor() as u8;
    rest_timestamp -= hour as f64 * SECONDS_IN_HOUR;
    let minute = (rest_timestamp / SECONDS_IN_MINUTE as f64).floor() as u8;
    rest_timestamp -= minute as f64 * SECONDS_IN_MINUTE as f64;
    let second = rest_timestamp.floor() as u8;
    let rest = rest_timestamp - second as f64;
    Time::from((hour, minute, second, rest))
}

#[allow(unused_assignments)]
pub fn make_now_date(timestamp: f64) -> (Date, f64, f64) {
    let mut tmp_timestamp = timestamp.clone();

    let years_since_epoch = ((timestamp / SECONDS_IN_DAY).trunc() / DAYS_IN_YEAR_APPROX).trunc();
    let mut leap_years = leap_years_since_epoch(years_since_epoch as u16);
    let mut year = EPOCH_YEAR + years_since_epoch as u16;
    tmp_timestamp -= years_since_epoch * SECONDS_IN_YEAR;

    let days_this_year = (tmp_timestamp / SECONDS_IN_DAY).trunc();
    //tmp_timestamp -= leap_years as f64 * SECONDS_IN_DAY;
    tmp_timestamp -= days_this_year as f64 * SECONDS_IN_DAY;

    let mut month: u8 = 0;
    let mut days_into_the_year: u16 = 0;
    while (days_into_the_year as f64) < days_this_year {
        days_into_the_year += NUMBER_OF_DAYS_PER_MONTH[month as usize] as u16;
        month += 1;
    }
    let completed_months = month.saturating_sub(1);
    let completed_month_days = {
        let mut out = 0;
        for i in 0..completed_months {
            out += NUMBER_OF_DAYS_PER_MONTH[i as usize] as u16;
        }
        out
    };

    // Fix for the 0.th month
    if month == 0 {
        month = 1;
    }

    if month <= 2 && is_this_year_leap_year(year) {
        leap_years -= 1;
    }

    debug_assert!(days_into_the_year >= completed_month_days as u16);

    let mut days_left_in_month: i16 = days_this_year as i16 - completed_month_days as i16;

    // Because the 0.th is not the 1.st!
    days_left_in_month += 1;

    let mut day: u8 = {
        debug_assert!(days_left_in_month >= 1);
        debug_assert!(days_left_in_month <= 31);
        // expect: Ok, because previous logic ensures:
        // all completed month days have been counted and removed, meaning:
        // days_left_in_month > 0 and days_left_in_month < 32
        days_left_in_month.try_into().expect("Could not convert.")
    };

    // Leap year handler
    // taking care of edge case: leap_years > 365;
    let mut tmp_leap_year_store = leap_years;
    // remove full years of leap year days
    while tmp_leap_year_store as f64 >= DAYS_IN_YEAR_APPROX {
        if is_this_year_leap_year(year) {
            tmp_leap_year_store -= 1;
        }
        year -= 1;
        tmp_leap_year_store -= DAYS_IN_YEAR_APPROX as u16;
    }
    debug_assert!(tmp_leap_year_store < 365);

    // Now I have: 0 <= leap_years <= 364
    // if 0: no leap year days left. -> day stays the same, month stays the same
    // if > 0: at least one leap year day left
    //
    // taking care of edge case: 0 < leap_years < 365;
    let mut prev_month = 0;
    while tmp_leap_year_store > 0 {
        // loop through as many months as it takes
        if tmp_leap_year_store > day as u16 {
            tmp_leap_year_store -= day as u16;
            prev_month = month;
            month -= 1;
            if month == 0 {
                year -= 1;
                prev_month = 1;
                month = 12;
            }
            if month <= 2 && is_this_year_leap_year(year) && prev_month != 2 {
                tmp_leap_year_store -= 1;
            }
            day = days_in_month(month);
        } else {
            let new_day = day - tmp_leap_year_store as u8;
            if new_day == 0 {
                prev_month = month;
                month -= 1;
                if month <= 2 && is_this_year_leap_year(year) && prev_month != 2 {
                    if prev_month != 2 {
                        tmp_leap_year_store -= 1;
                    }
                }
                if month == 0 {
                    year -= 1;
                    // I know its not read, unused_assignments flag is only for line below!
                    prev_month = 1;
                    month = 12;
                    day = days_in_month(month);
                } else {
                    day = days_in_month(month);
                }
            }
            day -= tmp_leap_year_store as u8;
            tmp_leap_year_store = 0;
        }
    }

    // now at most 24 hours are left
    debug_assert!(tmp_timestamp <= SECONDS_IN_DAY);
    let date = Date::from((year, month, day));
    (date, tmp_timestamp, timestamp)
}
