#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone};
    #[test]
    fn between_weeks_test() {
        let start = Local.ymd(2022, 9, 1);
        let end = Local.ymd(2022, 10, 1);
        let weeks = between_weeks((start, end));
        assert_eq!(weeks, 4);
    }
    #[test]
    fn between_weeks_with_time_test() {
        let start = Local.ymd(2022, 9, 5).and_hms(12, 0, 0);
        let end = Local.ymd(2022, 9, 26).and_hms(0, 0, 0);
        let weeks = between_weeks((start, end));
        assert_eq!(weeks, 2);
    }
}

use crate::date::get_duration::get_duration;
use chrono::Duration;

/// 计算一个时间段内包含的星期数量
///
/// # Arguments
///
/// * `date_range`: 时间段
///
/// returns: i64 内含的星期数
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use hutools::date::between_weeks;
/// let start = Local.ymd(2022,9,1);
/// let end = Local.ymd(2022,10,1);
/// let days = between_weeks((start,end));
/// assert_eq!(days,4)
/// ```
pub fn between_weeks<T>(date_range: (T, T)) -> i64
where
    T: std::ops::Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_weeks()
}
