

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

// 300k ns measured once
// 400k ns measured once
#[test]
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
fn from_ymd_hms_with_timezone() {
    let dt = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::CentralEuropeanSummerTime);
    // Rollover positive CEST(+02:00)
    assert_eq!("2022-01-01 02:59:59.000", dt.to_string());

    let chatham = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    //Rollover positive Chatham(+13.75)
    assert_eq!("2022-01-01 13:44:59.000", chatham.to_string());
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
