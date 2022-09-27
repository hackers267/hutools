use super::get_duration::get_duration;
use chrono::Duration;

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};

    use super::*;

    #[test]
    fn between_days_test() {
        let start = Local.ymd(2022, 9, 12);
        let end = Local.ymd(2022, 9, 20);
        let days = between_days((start, end));
        assert_eq!(days, 8)
    }
    #[test]
    fn between_days_with_time_test() {
        let start = Local.ymd(2022, 9, 12).and_hms(12, 0, 0);
        let end = Local.ymd(2022, 9, 20).and_hms(0, 0, 0);
        let days = between_days((start, end));
        assert_eq!(days, 7)
    }
}

///
/// 计算一个时间范围内的天数
/// # Arguments
///
/// * `date_range`: 时间范围
///
/// returns: i64 天数
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use hutools::date::between_days;
/// let start = Local.ymd(2000,8,12);
/// let end = Local.ymd(2000,8,20);
/// let days = between_days((start,end));
/// assert_eq!(days, 8);
/// ```
pub fn between_days<T>(date_range: (T, T)) -> i64
where
    T: std::ops::Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_days()
}
