use horae::{TimeZone, Utc};

const SECONDS_IN_MINUTE: u8 = 60;
const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_DAY: f64 = 86_400.0;
const SECONDS_IN_YEAR: f64 = 31_536_000.0;

#[test]
fn add_to_datetime_no_rollover() {
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
fn sub_from_datetime_no_rollover() {
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

#[test]
fn sub_from_datetime_rollover_hms() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 12, 1, 1, TimeZone::Utc);

    let duration_second = std::time::Duration::from_secs(2);
    let now_minus_second = utc_now - duration_second;
    assert_eq!("2020-02-02 12:00:59.000", now_minus_second.to_string());

    let duration_minute = std::time::Duration::from_secs(60 * 2);
    let now_minus_minute = utc_now - duration_minute;
    assert_eq!("2020-02-02 11:59:01.000", now_minus_minute.to_string());

    let duration_hour = std::time::Duration::from_secs((60 * 2) + ((60 * 60) * 12));
    let now_minus_hour = utc_now - duration_hour;
    assert_eq!("2020-02-01 23:59:01.000", now_minus_hour.to_string());

    let duration_day = std::time::Duration::from_secs(60 * 60 * 24);
    let now_minus_day = utc_now - duration_day;
    assert_eq!("2020-02-01 12:01:01.000", now_minus_day.to_string());

    // tests for months
    let duration_month = std::time::Duration::from_secs(60 * 60 * 24 * 31);
    let now_minus_month = utc_now - duration_month;
    assert_eq!("2020-01-02 12:01:01.000", now_minus_month.to_string());

    let duration_year = std::time::Duration::from_secs(60 * 60 * 24 * 365);
    let now_minus_year = utc_now - duration_year;
    assert_eq!("2019-02-02 12:01:01.000", now_minus_year.to_string());
}

#[test]
fn add_to_datetime_rollover_hms() {
    let utc_now = Utc::from_ymd_hms_timezone(2020, 02, 02, 22, 59, 59, TimeZone::Utc);

    let duration_second = std::time::Duration::from_secs(2);
    let now_plus_second = utc_now + duration_second;
    assert_eq!("2020-02-02 23:00:01.000", now_plus_second.to_string());

    let duration_minute = std::time::Duration::from_secs(60 * 2);
    let now_plus_minute = utc_now + duration_minute;
    assert_eq!("2020-02-02 23:01:59.000", now_plus_minute.to_string());

    let duration_hour = std::time::Duration::from_secs(60 * 60 + 1);
    let now_plus_hour = utc_now + duration_hour;
    assert_eq!("2020-02-03 00:00:00.000", now_plus_hour.to_string());

    let duration_day = std::time::Duration::from_secs(60 * 60 * 24);
    let now_plus_day = utc_now + duration_day;
    assert_eq!("2020-02-03 22:59:59.000", now_plus_day.to_string());

    let duration_month = std::time::Duration::from_secs(60 * 60 * 24 * 29);
    let now_plus_month = utc_now + duration_month;
    assert_eq!("2020-03-02 22:59:59.000", now_plus_month.to_string());

    let duration_year = std::time::Duration::from_secs(60 * 60 * 24 * 366);
    let now_plus_year = utc_now + duration_year;
    assert_eq!("2021-02-02 22:59:59.000", now_plus_year.to_string());
}
