use horae::Utc;

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

