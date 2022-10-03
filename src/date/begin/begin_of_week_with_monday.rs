use chrono::{Date, DateTime, Datelike, Duration, TimeZone};
use std::ops::Sub;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_week_with_monday_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 4);
        let result = begin_of_week_with_monday(date);
        assert_eq!(result, actual);
    }
    #[test]
    fn begin_of_week_with_monday_sunday_test() {
        let date = Utc.ymd(2008, 8, 11);
        let actual = Utc.ymd(2008, 8, 11);
        let result = begin_of_week_with_monday(date);
        assert_eq!(result, actual);
    }
    #[test]
    fn begin_of_week_with_monday_with_time_test() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 8, 4);
        let result = begin_of_week_with_monday_with_time(datetime);
        assert_eq!(actual, result)
    }
}

/// Get the start date of a week.
/// 获取某个星期的开始日期
/// # Arguments
///
/// * `date`:
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_week_with_monday;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,8,4);
/// let result = begin_of_week_with_monday(date);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_week_with_monday<Tz: TimeZone>(date: Date<Tz>) -> Date<Tz> {
    let num = date.weekday().number_from_monday() - 1;
    date.sub(Duration::days(num as i64))
}

/// Get the start date of a week.
/// 获取某个星期的开始日期
/// # Arguments
///
/// * `datetime`:
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_week_with_monday_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,4);
/// let result = begin_of_week_with_monday_with_time(datetime);
/// assert_eq!(result, actual);
/// ```
pub fn begin_of_week_with_monday_with_time<Tz: TimeZone>(datetime: DateTime<Tz>) -> Date<Tz> {
    begin_of_week_with_monday(datetime.date())
}
