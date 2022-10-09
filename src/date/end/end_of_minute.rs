use chrono::{DateTime, TimeZone, Timelike};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_end_of_minute() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let result = end_of_minute(datetime);
        let actual = Utc.ymd(2008, 8, 8).and_hms(8, 8, 59);
        assert_eq!(result, actual);
    }
}

/// Get the end of a minute.
/// 获取某分钟的结束时间
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
/// use hutools::date::end_of_minute;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let result = end_of_minute(datetime);
/// let actual = Utc.ymd(2008,8,8).and_hms(8,8,59);
/// assert_eq!(result, actual);
/// ```
pub fn end_of_minute<Tz>(datetime: DateTime<Tz>) -> DateTime<Tz>
where
    Tz: TimeZone,
{
    datetime.with_second(59).unwrap()
}
