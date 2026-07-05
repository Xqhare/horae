//! These should all fail, passed in are local date times without an offset.
//! They must not be parsed into a UTC date time.
use horae::Utc;

#[test]
fn datetime_0() {
    let dt = "2023-09-03T02:10:00.987";
    assert!(Utc::from_rfc3339(dt).is_none());
}

#[test]
fn datetime_1() {
    let dt = "1979-05-27T07:32:00";
    assert!(Utc::from_rfc3339(dt).is_none());
    let dt = "1979-05-27T07:32:00.05";
    assert!(Utc::from_rfc3339(dt).is_none());
    let dt = "1979-05-27T07:32:00.999999";
    assert!(Utc::from_rfc3339(dt).is_none());
}

#[test]
fn datetime_2() {
    let dt = "2023-09-03 02:10:00.987";
    assert!(Utc::from_rfc3339(dt).is_none());
}
