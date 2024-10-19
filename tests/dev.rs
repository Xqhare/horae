
use horae::Utc;

#[test]
fn creation_datetime_utc() {
    let dt = Utc::now();
    println!("{}", dt);
}
