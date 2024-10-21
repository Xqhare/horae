const SECONDS_IN_MINUTE: u8 = 60;
const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_DAY: f64 = 86_400.0;
const SECONDS_IN_YEAR: f64 = 31_536_000.0;

use horae::{TimeZone, Utc};

#[test]
fn main_creation_datetime_utc() {
    let instant = std::time::Instant::now();
    let dt = Utc::now();
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let dt2 = Utc::now();
    println!("{}", dt2);
}

// 300k ns measured once
// 400k ns measured once
#[test]
#[ignore]
/// > 15 sec
fn creation_time_performance() {
    let mut biggest_dur = 0;
    let mut loop_count = 0;
    let mut duration_sum = 0;
    for _ in 0..100_000_000 {
        let instant = std::time::Instant::now();
        let _dt = Utc::now();
        let tmp = instant.elapsed().as_nanos();

        if tmp > biggest_dur {
            biggest_dur = tmp;
        }
        loop_count += 1;
        duration_sum += tmp;
        if tmp > 500_000 {
            println!("Nanoseconds elapsed: {} in loop {}", tmp, loop_count);
        }
        assert!(tmp < 500_000);
    }
    println!("Average nanoseconds elapsed: {}", duration_sum / loop_count);
    println!("Most nanoseconds elapsed: {}", biggest_dur);
}

#[test]
fn creation_datetime_utc_with_timezone() {
    let instant = std::time::Instant::now();
    let mut dt = Utc::now();
    dt.with_timezone(TimeZone::CentralEuropeanSummerTime);
    println!("CEST: {}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let dt2 = Utc::now();
    println!("UTC: {}", dt2);
}


#[test]
fn from_ymd_hms_without_timezone() {
    let instant = std::time::Instant::now();
    let dt = Utc::from_ymd_hms(2021, 12, 31, 23, 59, 59);
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
}

#[test]
fn print_timezones() {
    println!("{}", TimeZone::IrkutskTime);
    println!("{}", TimeZone::CentralEuropeanSummerTime);
    println!("{}", TimeZone::ChathamDaylightTime);
    println!("{}", TimeZone::ChathamStandardTime);
    println!("{}", TimeZone::CentralAfricaTime);
    println!("{}", TimeZone::GreenwichMeanTime);
    println!("{}", TimeZone::VenezuelanStandardTime);
    println!("{}", TimeZone::Utc);
}

#[test]
fn from_ymd_hms_timezone_utc() {
    // utc(+00:00)
    let utc = Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::Utc);
    assert_eq!("2021-02-25 13:59:59.000", utc.to_string());
}

// no rollover positive
#[test]
fn timezone_positive() {
    // CEST(+02:00)
    let cest = Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::CentralEuropeanSummerTime);
    assert_eq!("2021-02-25 15:59:59.000", cest.to_string());
}

// no rollover negative
#[test]
fn timezone_negative() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-02-25 04:29:59.000", mart.to_string());
}

// negative rollover
#[test]
fn timezone_negative_rollover_year() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 01, 01, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2020-12-31 15:30:59.000", mart.to_string());
}

#[test]
fn timezone_negative_rollover_month() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 01, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-01-31 15:30:59.000", mart.to_string());
}

#[test]
fn timezone_negative_rollover_day() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 25, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-02-24 15:30:59.000", mart.to_string());
}

// positive rollover
#[test]
fn timezone_positive_rollover_year() {
    //Rollover positive Chatham(+13.75)
    let chatham = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2022-01-01 13:44:59.000", chatham.to_string());
}

#[test]
fn timezone_positive_rollover_month() {
    //Rollover positive Chatham(+13.75)
    let chatham = Utc::from_ymd_hms_timezone(2021, 03, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2021-04-01 13:44:59.000", chatham.to_string());
}

#[test]
fn timezone_positive_rollover_day() {
    //Rollover positive Chatham(+13.75)
    let chatham = Utc::from_ymd_hms_timezone(2021, 03, 28, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2021-03-29 13:44:59.000", chatham.to_string());
}

#[test]
fn add_duration_to_datetime_no_rollover() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 03, 02, 12, 0, 0, TimeZone::Utc); 

    println!("first");
    let duration_second = std::time::Duration::from_secs(1);
    let now_plus_second = utc_now + duration_second;
    assert_eq!("2020-03-02 12:00:01.000", now_plus_second.to_string());

    println!("second");
    let duration_minute = std::time::Duration::from_secs(SECONDS_IN_MINUTE.into());
    let now_plus_minute = utc_now + duration_minute;
    assert_eq!("2020-03-02 12:01:00.000", now_plus_minute.to_string());

    println!("third");
    let duration_hour = std::time::Duration::from_secs(SECONDS_IN_HOUR.trunc() as u64);
    let now_plus_hour = utc_now + duration_hour;
    assert_eq!("2020-03-02 13:00:00.000", now_plus_hour.to_string());

    println!("fourth");
    let duration_day = std::time::Duration::from_secs(SECONDS_IN_DAY.trunc() as u64);
    let now_plus_day = utc_now + duration_day;
    assert_eq!("2020-03-03 12:00:00.000", now_plus_day.to_string());

    println!("fifth");
    let duration_month = std::time::Duration::from_secs(31 * SECONDS_IN_DAY.trunc() as u64);
    let now_plus_month = utc_now + duration_month;
    assert_eq!("2020-04-02 12:00:00.000", now_plus_month.to_string());

    println!("sixth");
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_plus_year = utc_now + duration_year;
    assert_eq!("2021-03-02 12:00:00.000", now_plus_year.to_string());
}

#[test]
fn sub_duration_from_datetime_no_rollover() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 12, 1, 1, TimeZone::Utc);

    println!("first");
    let duration_second = std::time::Duration::from_secs(1);
    let now_minus_second = utc_now - duration_second;
    assert_eq!("2020-02-02 12:01:00.000", now_minus_second.to_string());

    println!("second");
    let duration_minute = std::time::Duration::from_secs(SECONDS_IN_MINUTE.into());
    let now_minus_minute = utc_now - duration_minute;
    assert_eq!("2020-02-02 12:00:01.000", now_minus_minute.to_string());

    println!("third");
    let duration_hour = std::time::Duration::from_secs(SECONDS_IN_HOUR.trunc() as u64);
    let now_minus_hour = utc_now - duration_hour;
    assert_eq!("2020-02-02 11:01:01.000", now_minus_hour.to_string());

    println!("fourth");
    let duration_day = std::time::Duration::from_secs(SECONDS_IN_DAY.trunc() as u64);
    let now_minus_day = utc_now - duration_day;
    assert_eq!("2020-02-01 12:01:01.000", now_minus_day.to_string());

    println!("fifth");
    let duration_month = std::time::Duration::from_secs(31 * SECONDS_IN_DAY.trunc() as u64);
    let now_minus_month = utc_now - duration_month;
    assert_eq!("2020-01-02 12:01:01.000", now_minus_month.to_string());

    println!("sixth");
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_minus_year = utc_now - duration_year;
    assert_eq!("2019-02-02 12:01:01.000", now_minus_year.to_string());
}

#[test]
fn off_by_one_hunt_add() {
    println!("ADD to 2019");
    let utc_add = Utc::from_ymd_hms_timezone(2019, 02, 12, 12, 1, 1, TimeZone::Utc);
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_plus_year = utc_add + duration_year;
    assert_eq!("2020-02-12 12:01:01.000", now_plus_year.to_string());
}

#[test]
fn off_by_one_hunt_sub() {
    println!("SUB from 2020");
    let utc_sub = Utc::from_ymd_hms_timezone(2020, 02, 12, 12, 1, 1, TimeZone::Utc);
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_minus_year = utc_sub - duration_year;
    assert_eq!("2019-02-12 12:01:01.000", now_minus_year.to_string());
}

#[test]
fn off_by_one_hunt_sub2() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 12, 1, 1, TimeZone::Utc);
    let duration_second = std::time::Duration::from_secs(1);
    let now_minus_second = utc_now - duration_second;
    assert_eq!("2020-02-02 12:01:00.000", now_minus_second.to_string());
}

// TODO: Leap day tests
