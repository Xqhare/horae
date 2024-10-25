use horae::{TimeZone, Utc};

#[test]
fn main_creation_datetime_utc() {
    let instant = std::time::Instant::now();
    let dt = Utc::now();
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let dt2 = Utc::now();
    println!("{}", dt2);
}

