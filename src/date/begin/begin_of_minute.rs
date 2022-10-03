use chrono::{DateTime, TimeZone, Timelike};
#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    #[test]
    fn begin_of_minute_test() {
        let date_time = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 8, 8).and_hms(8, 8, 0);
        let result = begin_of_minute(date_time);
        assert_eq!(actual, result);
    }
}

/// Get the begin of minute
/// 获取某个分钟的开始时间
/// # Arguments
///
/// * `date_time`: datetime 日期时间
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_minute;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,8).and_hms(8,8,0);
/// let result = begin_of_minute(datetime);
/// assert_eq!(actual, result);
/// ```
pub fn begin_of_minute<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    let hour = datetime.hour();
    let minute = datetime.minute();
    datetime.date().and_hms(hour, minute, 0)
}
