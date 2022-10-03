use chrono::{Date, DateTime, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    #[test]
    fn begin_of_day_with_time_test() {
        let date_time = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let result = begin_of_day_with_time(date_time);
        let actual = Utc.ymd(2008, 8, 8).and_hms(0, 0, 0);
        assert_eq!(result, actual);
    }
    #[test]
    fn begin_of_day_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 8, 8).and_hms(0, 0, 0);
        let result = begin_of_day(date);
        assert_eq!(actual, result);
    }
}

/// 获取某天的开始时间
///
/// # Arguments
///
/// * `date`: 日期时间
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_day_with_time;
/// let now = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,8).and_hms(0,0,0);
/// let result = begin_of_day_with_time(now);
/// assert_eq!(result, actual);
/// ```
pub fn begin_of_day_with_time<Tz: TimeZone>(date: DateTime<Tz>) -> DateTime<Tz> {
    date.date().and_hms(0, 0, 0)
}

/// 获取某天的开始时间
///
/// # Arguments
///
/// * `date`: 日期
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_day;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,8,8).and_hms(0,0,0);
/// let result = begin_of_day(date);
/// assert_eq!(result, actual);
/// ```
pub fn begin_of_day<Tz: TimeZone>(date: Date<Tz>) -> DateTime<Tz> {
    date.and_hms(0, 0, 0)
}
