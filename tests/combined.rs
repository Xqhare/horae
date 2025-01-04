use horae::{TimeZone, Utc};

#[test]
fn positive() {
    // CEST(+02:00)
    let cest = Utc::from_ymd_hms_timezone(
        2021,
        02,
        25,
        13,
        59,
        59,
        TimeZone::CentralEuropeanSummerTime,
    );
    assert_eq!("2021-02-25 15:59:59.000", cest.to_string());

    let now_plus_second = cest + std::time::Duration::from_secs(1);
    assert_eq!("2021-02-25 16:00:00.000", now_plus_second.to_string());

    let now_plus_minute = cest + std::time::Duration::from_secs(60);
    assert_eq!("2021-02-25 16:00:59.000", now_plus_minute.to_string());

    let now_plus_hour = cest + std::time::Duration::from_secs(3600);
    assert_eq!("2021-02-25 16:59:59.000", now_plus_hour.to_string());

    let now_plus_day = cest + std::time::Duration::from_secs(86400);
    assert_eq!("2021-02-26 15:59:59.000", now_plus_day.to_string());

    let now_plus_month = cest + std::time::Duration::from_secs(86400 * 31);
    assert_eq!("2021-03-28 15:59:59.000", now_plus_month.to_string());

    let now_plus_year = cest + std::time::Duration::from_secs(86400 * 365);
    assert_eq!("2022-02-25 15:59:59.000", now_plus_year.to_string());
}

#[test]
fn negative() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-02-25 04:29:59.000", mart.to_string());
    let second = std::time::Duration::from_secs(1);
    let now_plus_second = mart + second;
    assert_eq!("2021-02-25 05:30:00.000", now_plus_second.to_string());

    let now_minus_second = mart - second;
    assert_eq!("2021-02-25 04:29:58.000", now_minus_second.to_string());

    let now_minus_minute = mart - std::time::Duration::from_secs(60);
    assert_eq!("2021-02-25 04:28:59.000", now_minus_minute.to_string());

    let now_minus_hour = mart - std::time::Duration::from_secs(3600);
    assert_eq!("2021-02-25 03:29:59.000", now_minus_hour.to_string());

    let now_minus_day = mart - std::time::Duration::from_secs(86400);
    assert_eq!("2021-02-24 04:29:59.000", now_minus_day.to_string());

    let now_minus_month = mart - std::time::Duration::from_secs(86400 * 31);
    assert_eq!("2021-01-25 04:29:59.000", now_minus_month.to_string());

    let now_minus_year = mart - std::time::Duration::from_secs(86400 * 365);
    assert_eq!("2020-02-26 04:29:59.000", now_minus_year.to_string());
}
