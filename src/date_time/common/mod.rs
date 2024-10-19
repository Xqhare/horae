use crate::date_time::date::Date;

use super::time::Time;

const HOURS_IN_DAY: u8 = 24;
const MINUTES_IN_HOUR: u8 = 60;
const SECONDS_IN_MINUTE: u8 = 60;
const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_DAY: f64 = 86_400.0;
const SECONDS_IN_YEAR: f64 = 31_536_000.0;
const DAYS_IN_YEAR_APPROX: f64 = 365.0;
const EPOCH_YEAR: u16 = 1970;
const EPOCH_MONTH: u8 = 1;
const EPOCH_DAY: u8 = 1;
const EPOCH_HOUR: u8 = 0;
const EPOCH_MINUTE: u8 = 0;
const EPOCH_SECOND: u8 = 0;
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

/// I dont even know if I need this, leap seconds are not incrimenting the timestamp, it just
/// repeats a number...
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

pub fn make_now_date() -> (Date, f64, f64) {
    let unix_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
    let mut tmp_timestamp = unix_timestamp.clone();

    let days_since_epoch = unix_timestamp / SECONDS_IN_DAY;
    let years_since_epoch = (days_since_epoch / DAYS_IN_YEAR_APPROX).floor();
    let leap_years = leap_years_since_epoch(years_since_epoch as u16);
    println!("Leap years: {}", leap_years);
    let year = EPOCH_YEAR + years_since_epoch as u16;
    tmp_timestamp -= years_since_epoch * SECONDS_IN_YEAR;

    let days_this_year = (tmp_timestamp / SECONDS_IN_DAY).floor();
    println!("Days this year: {}", days_this_year);
    println!("Days this year float: {}", tmp_timestamp / SECONDS_IN_DAY);
    let mut month: u8 = 0;
    let mut days_into_the_year: u16 = 0;
    while (days_into_the_year as f64) < days_this_year {
        days_into_the_year += NUMBER_OF_DAYS_PER_MONTH[month.saturating_sub(1) as usize] as u16;
        month += 1;
    }
    let mut completed_months = month.saturating_sub(1);
    let mut completed_month_days = 0;
    for n in 0..completed_months {
        completed_month_days += NUMBER_OF_DAYS_PER_MONTH[n as usize] as u16;
    }
    debug_assert!(days_into_the_year >= completed_month_days);
    let days_left_in_month = days_into_the_year - completed_month_days;
    tmp_timestamp -= (completed_month_days as f64) * SECONDS_IN_DAY;
    let no_leaps_day: u8 = {
        if days_left_in_month == NUMBER_OF_DAYS_PER_MONTH[completed_months as usize] as u16 {
            // First day of next month
            completed_months += 1;
            tmp_timestamp -= NUMBER_OF_DAYS_PER_MONTH[completed_months as usize] as f64 * SECONDS_IN_DAY;
            1
        } else {
            // Any other day
            debug_assert!(days_left_in_month <= 31);
            tmp_timestamp -= days_left_in_month as f64 * SECONDS_IN_DAY;
            days_left_in_month.try_into().expect("Could not convert u16 to u8. Checked u16 to be smaller than 31")
        }
    };

    let day = {
        if (no_leaps_day as u16) <= leap_years {
            // We need to go back to the previous month
            completed_months -= 1;
            let days_in_previous_month = NUMBER_OF_DAYS_PER_MONTH[completed_months as usize] as u16;
            (days_in_previous_month - (leap_years - (no_leaps_day as u16))).try_into().expect("Could not convert u16 to u8. Checked u16 to be smaller than 31")
        } else {
            ((TryInto::<u16>::try_into(no_leaps_day).expect("Could not convert u16 to u8. Checked u16 to be smaller than 31")) - leap_years).try_into().expect("Could not convert u16 to u8. Checked u16 to be smaller than 31")
        }
    };
    
    // now at most 24 hours are left
    debug_assert!(tmp_timestamp <= SECONDS_IN_DAY);
    let date = Date::from_ymd(year, month, day);
    (date, tmp_timestamp, unix_timestamp)
}
