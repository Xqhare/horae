use horae::Utc;

#[test]
fn main_creation_datetime_utc() {
    let instant = std::time::Instant::now();
    let dt = Utc::now();
    println!("{}", dt);
    println!("Microseconds elapsed: {}", instant.elapsed().as_micros());
    let dt2 = Utc::now();
    println!("{}", dt2);
}

// TODO: add hour, minute and second rollovers both positive and negative

// TODO: add sub duration with rollover

// TODO: Leap day tests
// time - duration = on leap day
//   - ofc using seconds, minutes, hours, days, months, years
// time + duration = on leap day

// TODO: Combined rollover / no rollover tests
// - several of seconds, minutes, hours, days, months, years
// - and with timezones
