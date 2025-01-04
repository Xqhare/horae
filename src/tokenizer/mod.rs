mod tokens;

pub use tokens::Separator;
pub use tokens::Token;
pub use tokens::Unit;

/// Tokensize the format argument from `.format()`
pub fn tokenize<S: Into<String>>(format: S) -> Vec<Token> {
    let tmp_into_bind: String = format.into();
    let sepatated = tmp_into_bind.split("%");
    let mut generated_tokens = Vec::new();
    for token in sepatated {
        if token.contains("MS") {
            let rest = token.replace("MS", "");
            generated_tokens.push(Token::Unit(Unit::Millisecond));
            generated_tokens.push(Token::Separator(Separator {
                separator_symbol: rest,
            }));
        } else if token.contains("tz") {
            let rest = token.replace("tz", "");
            generated_tokens.push(Token::Unit(Unit::Timezone));
            generated_tokens.push(Token::Separator(Separator {
                separator_symbol: rest,
            }));
        } else if token.contains("m") {
            if token.contains("mmmm") {
                let rest = token.replace("m", "");
                generated_tokens.push(Token::Unit(Unit::WordMonth));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else if token.contains("mmm") {
                let rest = token.replace("m", "");
                generated_tokens.push(Token::Unit(Unit::ShortWordMonth));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else if token.contains("mm") {
                let rest = token.replace("m", "");
                generated_tokens.push(Token::Unit(Unit::NumMonth));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("m", "");
                generated_tokens.push(Token::Unit(Unit::ShortNumMonth));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("M") {
            if token.contains("MM") {
                let rest = token.replace("M", "");
                generated_tokens.push(Token::Unit(Unit::Minute));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("M", "");
                generated_tokens.push(Token::Unit(Unit::ShortMinute));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("wd") {
            if token.contains("wdd") {
                let rest = token.replace("wdd", "");
                generated_tokens.push(Token::Unit(Unit::WeekDay));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("wd", "");
                generated_tokens.push(Token::Unit(Unit::ShortWeekDay));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("d") {
            if token.contains("dd") {
                let rest = token.replace("d", "");
                generated_tokens.push(Token::Unit(Unit::Day));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("d", "");
                generated_tokens.push(Token::Unit(Unit::ShortDay));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("H") {
            if token.contains("HH") {
                let rest = token.replace("H", "");
                generated_tokens.push(Token::Unit(Unit::Hour));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("H", "");
                generated_tokens.push(Token::Unit(Unit::ShortHour));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("y") {
            if token.contains("yyyy") {
                let rest = token.replace("y", "");
                generated_tokens.push(Token::Unit(Unit::FullYear));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else if token.contains("yy") {
                let rest = token.replace("y", "");
                generated_tokens.push(Token::Unit(Unit::Year));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("y", "");
                generated_tokens.push(Token::Unit(Unit::ShortYear));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        } else if token.contains("S") {
            if token.contains("SS") {
                let rest = token.replace("S", "");
                generated_tokens.push(Token::Unit(Unit::Second));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            } else {
                let rest = token.replace("S", "");
                generated_tokens.push(Token::Unit(Unit::ShortSecond));
                generated_tokens.push(Token::Separator(Separator {
                    separator_symbol: rest,
                }));
            }
        }
    }
    generated_tokens
}
