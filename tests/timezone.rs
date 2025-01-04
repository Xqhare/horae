use horae::{TimeZone, Utc};

#[test]
fn from_ymd_hms_utc() {
    // utc(+00:00)
    let utc =
        Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::CoordinatedUniversalTime);
    assert_eq!("2021-02-25 13:59:59.000", utc.to_string());
}

// no rollover positive
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
}

// no rollover negative
#[test]
fn negative() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 25, 13, 59, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-02-25 04:29:59.000", mart.to_string());
}

// negative rollover
#[test]
fn negative_rollover_year() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 01, 01, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2020-12-31 15:30:59.000", mart.to_string());
}

#[test]
fn negative_rollover_month() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 01, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-01-31 15:30:59.000", mart.to_string());
}

#[test]
fn negative_rollover_day() {
    // Marquesas Islands =-09:30
    let mart = Utc::from_ymd_hms_timezone(2021, 02, 25, 0, 0, 59, TimeZone::MarquesasIslandsTime);
    assert_eq!("2021-02-24 15:30:59.000", mart.to_string());
}

// positive rollover
#[test]
fn positive_rollover_year() {
    //Rollover positive Chatham(+13.75)
    let chatham =
        Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2022-01-01 13:44:59.000", chatham.to_string());
}

#[test]
fn positive_rollover_month() {
    //Rollover positive Chatham(+13.75)
    let chatham =
        Utc::from_ymd_hms_timezone(2021, 03, 31, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2021-04-01 13:44:59.000", chatham.to_string());
}

#[test]
fn positive_rollover_day() {
    //Rollover positive Chatham(+13.75)
    let chatham =
        Utc::from_ymd_hms_timezone(2021, 03, 28, 23, 59, 59, TimeZone::ChathamDaylightTime);
    assert_eq!("2021-03-29 13:44:59.000", chatham.to_string());
}
