use chrono::{Date, DateTime, Datelike, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_month_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 1);
        let result = begin_of_month(date);
        assert_eq!(actual, result);
    }

    #[test]
    fn begin_of_month_with_time_test() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 8, 1);
        let result = begin_of_month_with_time(datetime);
        assert_eq!(actual, result);
    }
}
/// Get the start date of month
/// 获取某个月的开始日期
/// # Arguments
///
/// * `date`: 日期
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_month;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,8,1);
/// let result = begin_of_month(date);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_month<Tz: TimeZone>(date: Date<Tz>) -> Date<Tz> {
    date.with_day(1).unwrap()
}

/// Get the start date of month time.
/// 获取某个月的开始日期
/// # Arguments
///
/// * `datetime`: datetime（日期时间）
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_month_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,1);
/// let result = begin_of_month_with_time(datetime);
/// assert_eq!(result, actual);
/// ```
pub fn begin_of_month_with_time<Tz: TimeZone>(datetime: DateTime<Tz>) -> Date<Tz> {
    datetime.date().with_day(1).unwrap()
}
