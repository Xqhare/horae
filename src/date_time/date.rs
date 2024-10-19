
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
}

// Display implementation

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
