#[derive(Debug, Clone)]
pub enum Token {
    Separator(Separator),
    Unit(Unit),
}

#[derive(Debug, Clone)]
pub struct Separator {
    pub separator_symbol: String,
}

#[derive(Debug, Clone)]
pub enum Unit {
    Millisecond,
    ShortSecond,
    Second,
    ShortMinute,
    Minute,
    ShortHour,
    Hour,
    ShortDay,
    Day,
    ShortNumMonth,
    NumMonth,
    ShortWordMonth,
    WordMonth,
    ShortYear,
    Year,
    FullYear,
}
