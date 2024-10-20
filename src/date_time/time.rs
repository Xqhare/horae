
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub subseconds: u64,
}

impl Time {
    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour, minute, second, subseconds: 0 }
    }

    pub fn from_hmsns(hour: u8, minute: u8, second: u8, rest: f64) -> Time {
        let tmp = rest.fract() * 1_000_000_000.0;
        Time { hour, minute, second, subseconds: tmp.floor() as u64 }
    }
}

// Display implementation

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}.{:03}", self.hour, self.minute, self.second, self.subseconds)
    }
}
