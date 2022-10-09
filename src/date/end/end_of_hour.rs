use chrono::{DateTime, TimeZone, Timelike};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_end_of_hour() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let result = end_of_hour(datetime);
        let actual = Utc.ymd(2008, 8, 8).and_hms(8, 59, 59);
        assert_eq!(result, actual);
    }
}

/// 获取某个小时的结束时间
///
/// # Arguments
///
/// * `datetime`: 日期时间
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::end_of_hour;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let result = end_of_hour(datetime);
/// let actual = Utc.ymd(2008,8,8).and_hms(8,59,59);
/// assert_eq!(result, actual);
/// ```
pub fn end_of_hour<Tz>(datetime: DateTime<Tz>) -> DateTime<Tz>
where
    Tz: TimeZone,
{
    datetime.with_minute(59).unwrap().with_second(59).unwrap()
}
