use crate::tokenizer::{tokenize, Token, Unit};

#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<(u16, u8, u8)> for Date {
    fn from((year, month, day): (u16, u8, u8)) -> Date {
        Date { year, month, day }
    }
}

impl Date {
    pub fn format(&self, formatter: &str) -> String {
        let format_tokens = tokenize(formatter);
        let mut formatted_string = String::new();
        for token in format_tokens {
            match token {
                Token::Unit(unit) => match unit {
                    Unit::ShortDay => {
                        formatted_string.push_str(&format!("{:01}", self.day));
                    },
                    Unit::Day => {
                        formatted_string.push_str(&format!("{:02}", self.day));
                    },
                    Unit::ShortNumMonth => {
                        formatted_string.push_str(&format!("{:01}", self.month));
                    },
                    Unit::NumMonth => {
                        formatted_string.push_str(&format!("{:02}", self.month));
                    },
                    Unit::ShortWordMonth => {
                        const MONTHS: [&str; 12] = [
                            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
                            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
                        ];
                        formatted_string.push_str(&MONTHS[self.month as usize - 1]);

                    },
                    Unit::WordMonth => {
                        const MONTHS: [&str; 12] = [
                            "January", "February", "March", "April", "May", "June",
                            "July", "August", "September", "October", "November", "December",
                        ];
                        formatted_string.push_str(&MONTHS[self.month as usize - 1]);
                    },
                    Unit::ShortYear => {
                        formatted_string.push_str(&format!("{:01}", self.year.to_string().chars().last().expect("No Year found!")));},
                    Unit::Year => {
                        let year: String = self.year.to_string().chars().rev().take(2).collect();
                        formatted_string.push_str(&year);
                    },
                    Unit::FullYear => {
                        formatted_string.push_str(&format!("{}", self.year));
                    },
                    // Dont want to intruduce an error state now...
                    _ => {
                        formatted_string.push_str(" Date only supports Day, Month and Year ");
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
