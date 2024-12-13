#[derive(Debug, Clone)]
/// All Tokens needed to construct a formatted string
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
    Timezone,
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
    ShortWeekDay,
    WeekDay,
}
