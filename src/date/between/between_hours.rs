#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone};

    #[test]
    fn between_hours_test() {
        let start = Local.ymd(2022, 9, 12);
        let end = Local.ymd(2022, 9, 20);
        let hours = between_hours((start, end));
        assert_eq!(hours, 192)
    }
    #[test]
    fn between_hours_time_test() {
        let start = Local.ymd(2000, 8, 10).and_hms(18, 0, 0);
        let end = Local.ymd(2000, 8, 12).and_hms(19, 0, 0);
        let hours = between_hours((start, end));
        assert_eq!(hours, 49)
    }
}

use crate::date::get_duration::get_duration;
use chrono::Duration;

/// 计算一个时间范围内的小时数
///
/// # Arguments
///
/// * `date_range`: 时间范围
///
/// returns: i64 小时数
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use hutools::date::between_hours;
/// let start = Local.ymd(2000,8,12);
/// let end = Local.ymd(2000,8,18);
/// let hours = between_hours((start,end));
/// assert_eq!(hours,144);
/// ```
pub fn between_hours<T>(date_range: (T, T)) -> i64
where
    T: std::ops::Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_hours()
}
