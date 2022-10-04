use crate::date::get_duration::get_duration;
use chrono::Duration;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_between_seconds() {
        let start = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let end = Utc.ymd(2008, 8, 8).and_hms(20, 8, 8);
        let date_range = (start, end);
        let result = between_seconds(date_range);
        let actual = 12 * 60 * 60;
        assert_eq!(result, actual);
    }
}

/// calculate seconds between a date_range.
/// 获取一个时间段内包含的秒数
/// # Arguments
///
/// * `date_range`:
///
/// returns: i64
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::between_seconds;
/// let start = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let end = Utc.ymd(2008,8,8).and_hms(20,8,8);
/// let date_range = (start, end);
/// let result = between_seconds(date_range);
/// let actual = 12*60*60;
/// assert_eq!(result, actual);
/// ```
pub fn between_seconds<T>(date_range: (T, T)) -> i64
where
    T: std::ops::Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_seconds()
}
