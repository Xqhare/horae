use crate::tokenizer::{tokenize, Token, Unit};

use super::common::week_day;

#[derive(Debug, Copy, Clone)]
/// Contains all date information
///
/// Namely: year, month, day
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    unix_timestamp: f64,
}

impl From<(u16, u8, u8, f64)> for Date {
    fn from((year, month, day, unix_timestamp): (u16, u8, u8, f64)) -> Date {
        Date {
            year,
            month,
            day,
            unix_timestamp,
        }
    }
}

impl Date {
    /// Used to format a date
    ///
    /// For more information on the available formatting syntax, see the README in the API chapter.
    ///
    /// # Examples
    /// ```rust
    /// use horae::Utc;
    ///
    /// let utc_now = Utc::from_ymd_hms(2019, 1, 1, 9, 9, 9);
    /// assert_eq!(utc_now.date().format("%yyyy-%mm-%dd"), "2019-01-01");
    /// ```
    pub fn format(&self, formatter: &str) -> String {
        let format_tokens = tokenize(formatter);
        let mut formatted_string = String::new();
        for token in format_tokens {
            match token {
                Token::Unit(unit) => match unit {
                    Unit::ShortDay => {
                        formatted_string.push_str(&format!("{:01}", self.day));
                    }
                    Unit::Day => {
                        formatted_string.push_str(&format!("{:02}", self.day));
                    }
                    Unit::ShortNumMonth => {
                        formatted_string.push_str(&format!("{:01}", self.month));
                    }
                    Unit::NumMonth => {
                        formatted_string.push_str(&format!("{:02}", self.month));
                    }
                    Unit::ShortWordMonth => {
                        const MONTHS: [&str; 12] = [
                            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
                            "Nov", "Dec",
                        ];
                        formatted_string.push_str(&MONTHS[self.month as usize - 1]);
                    }
                    Unit::WordMonth => {
                        const MONTHS: [&str; 12] = [
                            "January",
                            "February",
                            "March",
                            "April",
                            "May",
                            "June",
                            "July",
                            "August",
                            "September",
                            "October",
                            "November",
                            "December",
                        ];
                        formatted_string.push_str(&MONTHS[self.month as usize - 1]);
                    }
                    Unit::ShortYear => {
                        formatted_string.push_str(&format!(
                            "{:01}",
                            self.year
                                .to_string()
                                .chars()
                                .last()
                                .expect("No Year found!")
                        ));
                    }
                    Unit::Year => {
                        let year_tmp: String =
                            self.year.to_string().chars().rev().take(2).collect();
                        let year = year_tmp.chars().rev().collect::<String>();
                        formatted_string.push_str(&year);
                    }
                    Unit::FullYear => {
                        formatted_string.push_str(&format!("{}", self.year));
                    }
                    Unit::WeekDay => {
                        let week_day_num = week_day(*&self.unix_timestamp);
                        let week_day = match week_day_num {
                            1 => "Monday",
                            2 => "Tuesday",
                            3 => "Wednesday",
                            4 => "Thursday",
                            5 => "Friday",
                            6 => "Saturday",
                            7 => "Sunday",
                            // Should really never happen!
                            _ => "Error",
                        };
                        formatted_string.push_str(week_day);
                    }
                    Unit::ShortWeekDay => {
                        let week_day_num = week_day(*&self.unix_timestamp);
                        let week_day = match week_day_num {
                            1 => "Mon",
                            2 => "Tue",
                            3 => "Wed",
                            4 => "Thu",
                            5 => "Fri",
                            6 => "Sat",
                            7 => "Sun",
                            // Should really never happen!
                            _ => "Error",
                        };
                        formatted_string.push_str(week_day);
                    }
                    // Dont want to intruduce an error state now...
                    _ => {
                        formatted_string
                            .push_str(" Date only supports Day, Week day, Month and Year ");
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

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
