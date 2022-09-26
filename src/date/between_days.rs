use super::get_duration::get_duration;
use chrono::{Date, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Local;

    #[test]
    fn between_days_test() {
        let start = Local.ymd(2022, 9, 12);
        let end = Local.ymd(2022, 9, 20);
        let days = between_days((start, end));
        assert_eq!(days, 8)
    }
}

type DateRange<T> = (Date<T>, Date<T>);

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
pub fn between_days<T: TimeZone>(date_range: DateRange<T>) -> i64 {
    let duration = get_duration(date_range);
    duration.num_days()
}
