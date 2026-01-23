
#[test]
fn basics() {
    let small_ts = 1.0;
    let dt = horae::Utc::from_timestamp(small_ts);
    assert_eq!(dt.unix_timestamp(), small_ts);

    let large_ts = 1_234_567_890.0;
    let dt = horae::Utc::from_timestamp(large_ts);
    assert_eq!(dt.unix_timestamp(), large_ts);
}
