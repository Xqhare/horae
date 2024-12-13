use horae::{TimeZone, Utc};

#[test]
#[ignore]
/// > 15 sec in release mode
/// > 1 min in debug mode
fn creation_time_performance() {
    let mut biggest_dur = 0;
    let mut loop_count = 0;
    let mut duration_sum = 0;
    for _ in 0..100_000_000 {
        let instant = std::time::Instant::now();
        let _dt = Utc::now();
        let tmp = instant.elapsed().as_nanos();

        if tmp > biggest_dur {
            biggest_dur = tmp;
        }
        loop_count += 1;
        duration_sum += tmp;
        if tmp > 500_000 {
            println!("Nanoseconds elapsed: {} in loop {}", tmp, loop_count);
        }
        assert!(tmp < 500_000);
    }
    println!("Average nanoseconds elapsed: {}", duration_sum / loop_count);
    println!("Most nanoseconds elapsed: {}", biggest_dur);
}

#[test]
fn creation_datetime_utc_with_timezone() {
    let instant = std::time::Instant::now();
    let mut dt = Utc::now();
    dt.with_timezone(TimeZone::CentralEuropeanSummerTime);
    assert!(instant.elapsed().as_nanos() < 500_000_000);
    let dt2 = Utc::now();
    assert!(instant.elapsed().as_nanos() < 1_000_000_000);
    // like idk, just assert smth too make sure dt2 is kept alive for a short while
    assert!(dt2.to_string().contains(":"))
}

#[test]
fn from_ymd_hms_without_timezone() {
    let instant = std::time::Instant::now();
    let dt = Utc::from_ymd_hms(2021, 12, 31, 23, 59, 59);
    assert_eq!("2021-12-31 23:59:59.000".to_string(), format!("{}", dt));
    assert!(instant.elapsed().as_nanos() < 500_000_000);
}
