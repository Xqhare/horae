use horae::{TimeZone, Utc};

#[test]
fn formatter_utc() {
    let datetime = Utc::from_ymd_hms(1997, 1, 15, 7, 30, 59);
    assert_eq!("1997-01-15 07:30:59", datetime.format("%yyyy-%mm-%dd %HH:%MM:%SS"));
    assert_eq!("7-1-15 7:30:59.000", datetime.format("%y-%m-%d %H:%M:%S.%MS"));
    assert_eq!("97-Jan-15 07:30:59.000", datetime.format("%yy-%mmm-%dd %HH:%MM:%SS.%MS"));
    assert_eq!("1997-January-15 07:30:59.000", datetime.format("%yyyy-%mmmm-%dd %HH:%MM:%SS.%MS"));
    assert_eq!("000,59;30;7", datetime.format("%MS,%S;%MM;%H"));
    assert_eq!("15/1::07|30", datetime.format("%d/%m::%HH|%MM"));

}

#[test]
fn formatter_weekday() {
    let dt1 = Utc::from_ymd_hms(1997, 1, 15, 7, 30, 59);
    assert_eq!("Wednesday", dt1.format("%wdd"));
    let truth = vec![
        ("Wed", "Wednesday", 1997, 1, 15),
        ("Thu", "Thursday", 1997, 1, 16),
        ("Fri", "Friday", 1997, 1, 17),
        ("Sat", "Saturday", 1997, 1, 18),
        ("Sun", "Sunday", 1997, 1, 19),
        ("Mon", "Monday", 1997, 1, 20),
        ("Tue", "Tuesday", 1997, 1, 21),
        ("Fri", "Friday", 1997, 4, 18),
        ("Sat", "Saturday", 1997, 4, 19),
        ("Sun", "Sunday", 1997, 4, 20),
        ("Tue", "Tuesday", 2016, 3, 1),
        ("Wed", "Wednesday", 2016, 3, 2),
        ("Thu", "Thursday", 2016, 3, 3),
        ("Fri", "Friday", 1994, 9, 30),
        ("Sat", "Saturday", 1994, 10, 1),
        ("Sun", "Sunday", 1994, 10, 2),
        ("Tue", "Tuesday", 1979, 10, 16),
        ("Wed", "Wednesday", 1979, 10, 17),
        ("Sat", "Saturday", 1989, 4, 22),
        ("Sun", "Sunday", 1989, 4, 23),
    ];
    for t in truth {
        let dt = Utc::from_ymd_hms(t.2, t.3, t.4, 0, 0, 0);
        assert_eq!(t.0, dt.format("%wd"));
        assert_eq!(t.1, dt.format("%wdd"));
    }
}

#[test]
fn formatter_date() {
    let datetime = Utc::from_ymd_hms(1997, 1, 15, 7, 3, 5);
    let date = datetime.date();

    assert_eq!("15", date.format("%dd"));
    assert_eq!("15", date.format("%d"));
    assert_eq!("15::", date.format("%dd::"));

    assert_eq!("1997-1-15", date.format("%yyyy-%m-%dd"));
    assert_eq!("1997-01-15", date.format("%yyyy-%mm-%dd"));
    assert_eq!("1997-Jan-15", date.format("%yyyy-%mmm-%dd"));
    assert_eq!("1997-January-15", date.format("%yyyy-%mmmm-%dd"));

    assert_eq!("7-1", date.format("%y-%m"));
    assert_eq!("97-01", date.format("%yy-%mm"));
    assert_eq!("97-01", date.format("%yyy-%mm"));
    assert_eq!("1997-Jan", date.format("%yyyy-%mmm"));

    assert_eq!("1997!January??15", date.format("%yyyy!%mmmm??%dd"));
}

#[test]
fn formatter_time() {
    let datetime = Utc::from_ymd_hms(1997, 1, 15, 7, 3, 5);
    let time = datetime.time();

    assert_eq!("07", time.format("%HH"));
    assert_eq!("7", time.format("%H"));
    assert_eq!("07:03", time.format("%HH:%MM"));
    assert_eq!("7:3", time.format("%H:%M"));
    assert_eq!("07::03", time.format("%HH::%MM"));
    assert_eq!("7!000?3", time.format("%H!%MS?%M"));
}

#[test]
fn formatter_timezone() {
    for tz in TimeZone::get_all() {
        let mut time1 = Utc::from_ymd_hms_timezone(2021, 12, 31, 23, 59, 59, tz);
        time1.with_timezone(tz);
        let time1_str = time1.format("%tz");
        assert_eq!(time1_str, tz.to_string());
    }
}
