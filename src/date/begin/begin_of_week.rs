use chrono::{Date, DateTime, Datelike, Duration, TimeZone, Weekday};
use std::ops::Sub;
#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_week_from_monday_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 4);
        let result = begin_of_week(date, Weekday::Mon);
        assert_eq!(result, actual);
    }

    #[test]
    fn begin_of_week_from_sunday_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 3);
        let result = begin_of_week(date, Weekday::Sun);
        assert_eq!(result, actual);
    }
}

/// Get the start date of a week from the specified weekday.
/// 获取某个星期的开始日期(以指定的星期那天为开始日)
/// # Arguments
///
/// * `date`:
/// * `from`:
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc, Weekday};
/// use hutools::date::begin_of_week;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,8,7);
/// let result = begin_of_week(date,Weekday::Thu);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_week<Tz: TimeZone>(date: Date<Tz>, from: Weekday) -> Date<Tz> {
    let date_weekday = date.weekday();
    let num = date_weekday.number_from_monday();
    let num2 = from.number_from_monday();
    let diff = (num + 7 - num2) % 7;
    date.sub(Duration::days(diff as i64))
}

/// Get the start date of a week from the specified weekday.
/// 获取某个星期的开始日期(以指定的星期那天为开始日)
/// # Arguments
///
/// * `datetime`:
/// * `from`:
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc, Weekday};
/// use hutools::date::begin_of_week_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,8);
/// let result = begin_of_week_with_time(datetime,Weekday::Fri);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_week_with_time<Tz: TimeZone>(datetime: DateTime<Tz>, from: Weekday) -> Date<Tz> {
    begin_of_week(datetime.date(), from)
}
