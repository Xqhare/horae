
use std::thread::sleep;

use horae::Utc;

#[test]
fn creation_datetime_utc() {
    let dt = Utc::now();
    println!("{}", dt);
    sleep(std::time::Duration::from_nanos(10));
    let dt2 = Utc::now();
    println!("{}", dt2);
}
