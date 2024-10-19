
use std::thread::sleep;

use horae::Utc;

#[test]
fn creation_datetime_utc() {
    let instant = std::time::Instant::now();
    let dt = Utc::now();
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    sleep(std::time::Duration::from_micros(10));
    let dt2 = Utc::now();
    println!("{}", dt2);
}
