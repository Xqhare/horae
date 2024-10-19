
/// TODO: add all the timezones
pub enum TimeZone {
    UTC,
    GMT,
    CEST,
    CET,
}

impl From<&str> for TimeZone {
    fn from(timezone: &str) -> Self {
        match timezone {
            "UTC" => TimeZone::UTC,
            "GMT" => TimeZone::GMT,
            "CET" => TimeZone::CET,
            "CEST" => TimeZone::CEST,
            _ => TimeZone::UTC,
        }
    }
}
