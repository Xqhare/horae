

use horae::{TimeZone, Utc};

#[test]
fn creation_datetime_utc() {
    let instant = std::time::Instant::now();
    let dt = Utc::now();
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let dt2 = Utc::now();
    println!("{}", dt2);
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
fn from_ymd_hms_with_timezone() {
    let instant = std::time::Instant::now();
    let dt = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::CentralEuropeanSummerTime);
    println!("Rollover positive CEST: {}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let chatham = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    println!("Rollover positive Chatham(+13.75): {}", chatham);
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

// TODO Test negative utc offsets and rollovers
