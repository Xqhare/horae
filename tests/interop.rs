use horae::Utc;

#[test]
fn test_f64_interop() {
    let timestamp = 1647081600.0;
    let utc = Utc::from_timestamp(timestamp);

    // Test From<Utc> for f64
    let back_to_f64: f64 = utc.into();
    assert_eq!(back_to_f64, timestamp);

    // Test From<f64> for Utc
    let from_f64 = Utc::from(timestamp);
    assert_eq!(from_f64.unix_timestamp(), timestamp);
}

#[test]
fn test_system_time_interop() {
    use std::time::{Duration, UNIX_EPOCH};
    let timestamp = 1647081600.0;
    let st = UNIX_EPOCH + Duration::from_secs_f64(timestamp);

    // Test From<SystemTime> for Utc
    let utc = Utc::from(st);
    assert_eq!(utc.unix_timestamp(), timestamp);
}
