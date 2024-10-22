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
    assert!(instant.elapsed().as_nanos() < 500_000_000);
    let dt2 = Utc::now();
    assert!(instant.elapsed().as_nanos() < 1_000_000_000);
    // like idk, just assert smth too make sure dt2 is kept alive for a short while
    assert!(dt2.to_string().contains(":"))
}


#[test]
fn from_ymd_hms_without_timezone() {
    let instant = std::time::Instant::now();
    let dt = Utc::from_ymd_hms(2021, 12, 31, 23, 59, 59);
    assert_eq!("2021-12-31 23:59:59.000".to_string(), format!("{}", dt));
    assert!(instant.elapsed().as_nanos() < 500_000_000);
}

#[test]
fn all_timezones() {
    let timezones = TimeZone::get_all();
    let tz_len = timezones.len();
    let mut test_vec: Vec<Utc> = Vec::new();
    for tz in timezones {
        let mut utc = Utc::now();
        utc.with_timezone(tz);
        test_vec.push(utc);
    }
    // really I am looking for crashes, I have no way of confirming the validity of the resulting
    // DateTime
    assert_eq!(tz_len, test_vec.len());
}

#[test]
fn print_timezones() {
    assert_eq!("Irkutsk Time".to_string(), TimeZone::IrkutskTime.to_string());
    assert_eq!("Central European Summer Time".to_string(), TimeZone::CentralEuropeanSummerTime.to_string());
    assert_eq!("Chatham Daylight Time".to_string(), TimeZone::ChathamDaylightTime.to_string());
    assert_eq!("Chatham Standard Time".to_string(), TimeZone::ChathamStandardTime.to_string());
    assert_eq!("Central Africa Time".to_string(), TimeZone::CentralAfricaTime.to_string());
    assert_eq!("Greenwich Mean Time".to_string(), TimeZone::GreenwichMeanTime.to_string());
    assert_eq!("Venezuelan Standard Time".to_string(), TimeZone::VenezuelanStandardTime.to_string());
    assert_eq!("Coordinated Universal Time".to_string(), TimeZone::Utc.to_string());
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

// TODO: add hour, minute and second rollovers both positive and negative
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

    let duration_second = std::time::Duration::from_secs(1);
    let now_plus_second = utc_now + duration_second;
    assert_eq!("2020-03-02 12:00:01.000", now_plus_second.to_string());

    let duration_minute = std::time::Duration::from_secs(SECONDS_IN_MINUTE.into());
    let now_plus_minute = utc_now + duration_minute;
    assert_eq!("2020-03-02 12:01:00.000", now_plus_minute.to_string());

    let duration_hour = std::time::Duration::from_secs(SECONDS_IN_HOUR.trunc() as u64);
    let now_plus_hour = utc_now + duration_hour;
    assert_eq!("2020-03-02 13:00:00.000", now_plus_hour.to_string());

    let duration_day = std::time::Duration::from_secs(SECONDS_IN_DAY.trunc() as u64);
    let now_plus_day = utc_now + duration_day;
    assert_eq!("2020-03-03 12:00:00.000", now_plus_day.to_string());

    let duration_month = std::time::Duration::from_secs(31 * SECONDS_IN_DAY.trunc() as u64);
    let now_plus_month = utc_now + duration_month;
    assert_eq!("2020-04-02 12:00:00.000", now_plus_month.to_string());

    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_plus_year = utc_now + duration_year;
    assert_eq!("2021-03-02 12:00:00.000", now_plus_year.to_string());

    let utc_add = Utc::from_ymd_hms_timezone(2019, 02, 12, 12, 1, 1, TimeZone::Utc);
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_plus_year = utc_add + duration_year;
    assert_eq!("2020-02-12 12:01:01.000", now_plus_year.to_string());
}

#[test]
fn sub_duration_from_datetime_no_rollover() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 12, 1, 1, TimeZone::Utc);

    let duration_second = std::time::Duration::from_secs(1);
    let now_minus_second = utc_now - duration_second;
    assert_eq!("2020-02-02 12:01:00.000", now_minus_second.to_string());

    let duration_minute = std::time::Duration::from_secs(SECONDS_IN_MINUTE.into());
    let now_minus_minute = utc_now - duration_minute;
    assert_eq!("2020-02-02 12:00:01.000", now_minus_minute.to_string());

    let duration_hour = std::time::Duration::from_secs(SECONDS_IN_HOUR.trunc() as u64);
    let now_minus_hour = utc_now - duration_hour;
    assert_eq!("2020-02-02 11:01:01.000", now_minus_hour.to_string());

    let duration_day = std::time::Duration::from_secs(SECONDS_IN_DAY.trunc() as u64);
    let now_minus_day = utc_now - duration_day;
    assert_eq!("2020-02-01 12:01:01.000", now_minus_day.to_string());

    let duration_month = std::time::Duration::from_secs(31 * SECONDS_IN_DAY.trunc() as u64);
    let now_minus_month = utc_now - duration_month;
    assert_eq!("2020-01-02 12:01:01.000", now_minus_month.to_string());

    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_minus_year = utc_now - duration_year;
    assert_eq!("2019-02-02 12:01:01.000", now_minus_year.to_string());

    let utc_sub = Utc::from_ymd_hms_timezone(2020, 02, 12, 12, 1, 1, TimeZone::Utc);
    let duration_year = std::time::Duration::from_secs(SECONDS_IN_YEAR.trunc() as u64);
    let now_minus_year = utc_sub - duration_year;
    assert_eq!("2019-02-12 12:01:01.000", now_minus_year.to_string());

    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 12, 1, 1, TimeZone::Utc);
    let duration_second = std::time::Duration::from_secs(1);
    let now_minus_second = utc_now - duration_second;
    assert_eq!("2020-02-02 12:01:00.000", now_minus_second.to_string());
}

// TODO: Leap day tests
