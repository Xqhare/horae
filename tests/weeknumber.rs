use horae::Utc;

#[test]
fn test_weeknumbers() {
    let cases = vec![
        (2026, 3, 12, 11),
        (2026, 1, 1, 1),
        (2026, 1, 4, 1),
        (2026, 1, 5, 2),
        // Edge case where last day of year is Wednesday in the first week of the new year.
        (2025, 12, 31, 1),
        (2024, 12, 30, 1),
        (2023, 7, 10, 28),
        (2022, 10, 3, 40),
        // Edge case where first day of year is Friday in the last week of the last year.
        (2021, 1, 1, 53),
        (2021, 1, 4, 1),
        (2020, 1, 1, 1),
        (2019, 12, 30, 1),
        (2019, 12, 29, 52),
    ];

    for (y, m, d, expected) in cases {
        let utc = Utc::from_ymd_hms(y, m, d, 0, 0, 0);
        assert_eq!(
            utc.get_weeknumber(),
            expected,
            "Failed for {}-{}-{}",
            y,
            m,
            d
        );
    }
}

#[test]
fn test_weeknumber_formatting() {
    let utc = Utc::from_ymd_hms(2026, 3, 12, 0, 0, 0);
    assert_eq!(utc.format("%wn"), "11");
    assert_eq!(utc.format("%wnn"), "11");

    let utc2 = Utc::from_ymd_hms(2026, 1, 1, 0, 0, 0);
    assert_eq!(utc2.format("%wn"), "1");
    assert_eq!(utc2.format("%wnn"), "01");
}

#[test]
fn test_weeknumber_timezone() {
    // Jan 1 2021 00:00:00 UTC is Friday. Week 53 of 2020.
    let utc = Utc::from_ymd_hms(2021, 1, 1, 0, 0, 0);
    assert_eq!(utc.get_weeknumber(), 53);

    // If we are in a timezone that is ahead, say GMT+13, it's still Jan 1.
    let mut utc_ahead = utc.clone();
    utc_ahead.with_utc_offset(13.0);
    assert_eq!(utc_ahead.get_weeknumber(), 53);

    // If we are in a timezone that is behind, say GMT-1, it's Dec 31 2020.
    // Dec 31 2020 is Thursday. Week 53 of 2020.
    let mut utc_behind = utc.clone();
    utc_behind.with_utc_offset(-1.0);
    assert_eq!(utc_behind.get_weeknumber(), 53);

    // Dec 30 2019 (Monday) is Week 1 of 2020.
    let utc_2019 = Utc::from_ymd_hms(2019, 12, 30, 0, 0, 0);
    assert_eq!(utc_2019.get_weeknumber(), 1);

    // Dec 29 2019 (Sunday) is Week 52 of 2019.
    let utc_2019_sun = Utc::from_ymd_hms(2019, 12, 29, 23, 0, 0);
    assert_eq!(utc_2019_sun.get_weeknumber(), 52);

    // If we add 2 hours (GMT+2), it becomes Dec 30 2019 01:00:00, which is Week 1.
    let mut utc_2019_sun_ahead = utc_2019_sun.clone();
    utc_2019_sun_ahead.with_utc_offset(2.0);
    assert_eq!(utc_2019_sun_ahead.get_weeknumber(), 1);
}
