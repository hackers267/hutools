use chrono::{Date, DateTime, Datelike, Duration, TimeZone};
use std::ops::Sub;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_week_with_sunday_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 3);
        let result = begin_of_week_with_sunday(date);
        assert_eq!(result, actual);
    }

    #[test]
    fn begin_of_week_with_sunday_test1() {
        let date = Utc.ymd(2008, 8, 3);
        let actual = Utc.ymd(2008, 8, 3);
        let result = begin_of_week_with_sunday(date);
        assert_eq!(result, actual);
    }

    #[test]
    fn begin_of_week_with_sunday_with_time_test() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 8, 3);
        let result = begin_of_week_with_sunday_with_time(datetime);
        assert_eq!(result, actual);
    }
}

/// Get the start of a week which start from sunday;
/// 获取某个星期的开始日期(以周日为第一天)
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
/// use hutools::date::begin_of_week_with_sunday;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,8,3);
/// let result = begin_of_week_with_sunday(date);
/// assert_eq!(result, actual);
/// ```
pub fn begin_of_week_with_sunday<Tz: TimeZone>(date: Date<Tz>) -> Date<Tz> {
    let num = date.weekday().number_from_sunday() - 1;
    date.sub(Duration::days(num as i64))
}

/// Get the start of a week which start from sunday;
/// 获取某个星期的开始日期(以周日为第一天)
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
/// use hutools::date::begin_of_week_with_sunday_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,3);
///let result = begin_of_week_with_sunday_with_time(datetime);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_week_with_sunday_with_time<Tz: TimeZone>(datetime: DateTime<Tz>) -> Date<Tz> {
    begin_of_week_with_sunday(datetime.date())
}
