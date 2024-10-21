

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
