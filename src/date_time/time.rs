
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    rest: f64,
}

impl Time {
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour, minute, second, rest: 0.0 }
    }

    pub fn from_hmsns(hour: u8, minute: u8, second: u8, rest: f64) -> Time {
        Time { hour, minute, second, rest }
    }
}

// Display implementation

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}.{:03}", self.hour, self.minute, self.second, self.rest)
    }
}
