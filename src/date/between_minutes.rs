use super::get_duration::get_duration;
use chrono::Duration;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone};
    #[test]
    fn between_minutes_test() {
        let start = Local.ymd(2022, 5, 1);
        let end = Local.ymd(2022, 5, 2);
        let minutes = between_minutes((start, end));
        assert_eq!(minutes, 1440);
    }
    #[test]
    fn between_minutes_with_time_test() {
        let start = Local.ymd(2022, 5, 1).and_hms(12, 11, 0);
        let end = Local.ymd(2022, 5, 2).and_hms(0, 10, 0);
        let minutes = between_minutes((start, end));
        assert_eq!(minutes, 719);
    }
}
/// 计算时间段内含有的分钟数
///
/// # Arguments
///
/// * `date_range`: 时间段
///
/// returns: i64 分钟数
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use hutools::date::between_minutes;
/// let start = Local.ymd(2022,8,1).and_hms(12,11,0);
/// let end = Local.ymd(2022,8,2).and_hms(0,0,0);
/// let minutes = between_minutes((start,end));
/// assert_eq!(minutes,709);
/// ```
pub fn between_minutes<T>(date_range: (T, T)) -> i64
where
    T: std::ops::Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_minutes()
}
