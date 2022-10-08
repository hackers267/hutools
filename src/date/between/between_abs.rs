use crate::date::{
    between_days, between_hours, between_minutes, between_months, between_ms, between_seconds,
    between_years,
};
use chrono::{Date, TimeZone};

pub enum Unit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Micro,
}

pub fn between<Tz>(date_range: (Date<Tz>, Date<Tz>), unit: Unit) -> Option<i64>
where
    Tz: TimeZone,
{
    match unit {
        Unit::Year => between_years(date_range),
        Unit::Month => between_months(date_range),
        Unit::Day => between_days(date_range),
        Unit::Hour => between_hours(date_range),
        Unit::Minute => between_minutes(date_range),
        Unit::Second => between_seconds(date_range),
        Unit::Micro => between_ms(date_range),
    }
}

pub fn between_abs<Tz>(date_range: (Date<Tz>, Date<Tz>), unit: Unit) -> Option<u64>
where
    Tz: TimeZone,
{
    let result = between(date_range, unit);
    result.map(|x| x.abs()).map(|result| result as u64)
}
