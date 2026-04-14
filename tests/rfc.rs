use horae::Utc;

#[test]
fn test_rfc3339_formatting() {
    let dt = Utc::from_ymd_hms(1985, 4, 12, 23, 20, 50);
    assert_eq!(dt.to_rfc3339(), "1985-04-12T23:20:50Z");

    let mut dt_offset = Utc::from_ymd_hms(1996, 12, 19, 16, 39, 57);
    dt_offset.with_utc_offset(-8.0);
    // Utc::from_ymd_hms creates a UTC time.
    // If called with_utc_offset(-8.0), it changes the timezone offset of the same UTC moment.
    assert_eq!(dt_offset.to_rfc3339(), "1996-12-19T08:39:57-08:00");

    // Test fractional seconds
    let dt_frac = Utc::from_timestamp(1130590.958881855);
    assert_eq!(dt_frac.to_rfc3339(), "1970-01-14T02:03:10.958881855Z");
}

#[test]
fn test_rfc3339_parsing() {
    let s = "1985-04-12T23:20:50.5Z";
    let dt = Utc::from_rfc3339(s).unwrap();
    assert_eq!(dt.to_rfc3339(), "1985-04-12T23:20:50.5Z");

    let s2 = "1996-12-19T16:39:57-08:00";
    let dt2 = Utc::from_rfc3339(s2).unwrap();
    assert_eq!(dt2.to_rfc3339(), "1996-12-19T16:39:57-08:00");

    // Mixed case T/Z
    let s3 = "1996-12-19t16:39:57z";
    let dt3 = Utc::from_rfc3339(s3).unwrap();
    assert_eq!(dt3.to_rfc3339(), "1996-12-19T16:39:57Z");
}

#[test]
fn test_rfc9557_formatting() {
    let mut dt = Utc::from_ymd_hms(1996, 12, 19, 16, 39, 57);
    dt.with_utc_offset(-8.0);
    assert_eq!(dt.to_rfc9557(), "1996-12-19T08:39:57-08:00[-08:00]");
}

#[test]
fn test_rfc9557_parsing() {
    let s = "1996-12-19T16:39:57-08:00[America/Los_Angeles]";
    let dt = Utc::from_rfc9557(s).unwrap(); // Elective unknown tag
    assert_eq!(dt.get_utc_offset(), -8.0);

    let s2 = "1996-12-19T16:39:57-08:00[!u-ca=hebrew]";
    let dt2 = Utc::from_rfc9557(s2);
    assert!(dt2.is_none()); // Critical unknown tag

    let s3 = "1996-12-19T16:39:57-08:00[u-ca=hebrew]";
    let dt3 = Utc::from_rfc9557(s3).unwrap(); // Elective unknown tag
    assert_eq!(dt3.get_utc_offset(), -8.0);

    let s4 = "2022-07-08T00:14:07+01:00[!+01:00]";
    let dt4 = Utc::from_rfc9557(s4).unwrap();
    assert_eq!(dt4.get_utc_offset(), 1.0);

    let s5 = "2022-07-08T00:14:07+01:00[!-08:00]";
    let dt5 = Utc::from_rfc9557(s5);
    assert!(dt5.is_none()); // Critical inconsistent offset
}
