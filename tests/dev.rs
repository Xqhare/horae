

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
