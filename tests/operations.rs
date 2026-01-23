
#[test]
fn sub_utc_utc() {
    let utc1 = horae::Utc::now();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let utc2 = horae::Utc::now();
    let duration = utc2 - utc1;
    assert!(duration.as_secs_f64() > 0.0);
}

#[test]
fn sub_utc_duration() {
    let utc = horae::Utc::now();
    let duration = std::time::Duration::from_secs_f64(1.0);
    let utc2 = utc - duration;
    assert!(utc2.unix_timestamp() < utc.unix_timestamp());
}

#[test]
fn add_duration_utc() {
    let utc = horae::Utc::now();
    let duration = std::time::Duration::from_secs_f64(1.0);
    let utc2 = utc + duration;
    assert!(utc2.unix_timestamp() > utc.unix_timestamp());
}
