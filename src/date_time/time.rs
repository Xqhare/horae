use crate::tokenizer::{tokenize, Token, Unit};

#[derive(Debug, Copy, Clone)]
/// Contains all time information
///
/// Holds the hour, minute, second and subseconds.
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub subseconds: u64,
}

impl From<(u8, u8, u8, f64)> for Time {
    fn from((hour, minute, second, subseconds): (u8, u8, u8, f64)) -> Time {
        let tmp = subseconds.fract() * 1_000_000_000.0;
        Time {
            hour,
            minute,
            second,
            subseconds: tmp.floor() as u64,
        }
    }
}

impl From<(u8, u8, u8)> for Time {
    fn from((hour, minute, second): (u8, u8, u8)) -> Time {
        Time {
            hour,
            minute,
            second,
            subseconds: 0,
        }
    }
}

impl Time {
    /// Formats the time in the given format
    /// For more information on the available formatting syntax, see the README in the API chapter.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.time().format("%HH:%MM:%SS"), "09:09:09");
    /// ```
    pub fn format(&self, formatter: &str) -> String {
        let format_tokens = tokenize(formatter);
        let mut formatted_string = String::new();
        for token in format_tokens {
            match token {
                Token::Unit(unit) => match unit {
                    Unit::Millisecond => {
                        formatted_string.push_str(&format!("{:03}", self.subseconds));
                    },
                    Unit::ShortSecond => {
                        formatted_string.push_str(&format!("{:01}", self.second));
                    },
                    Unit::Second => {
                        formatted_string.push_str(&format!("{:02}", self.second));    
                    },
                    Unit::ShortMinute => {
                        formatted_string.push_str(&format!("{:01}", self.minute));
                    },
                    Unit::Minute => {
                        formatted_string.push_str(&format!("{:02}", self.minute));
                    },
                    Unit::ShortHour => {
                        formatted_string.push_str(&format!("{:01}", self.hour));
                    },
                    Unit::Hour => {
                        formatted_string.push_str(&format!("{:02}", self.hour));
                    },
                    // Dont want to intruduce an error state now...
                    _ => {
                        formatted_string.push_str(" Time only supports millisecond, second, minute, and hour ");
                    }
                },
                Token::Separator(separator) => {
                    formatted_string.push_str(&separator.separator_symbol);
                }
            }
        }

        formatted_string
    }
}

// Display implementation

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hour, self.minute, self.second, self.subseconds
        )
    }
}
