#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Date, Local, TimeZone};
    #[test]
    fn days_test() {
        let date_range = get_date_range();
        let days = date_range.days();
        assert_eq!(days, 2)
    }
    #[test]
    fn hours_test() {
        let date_range = get_date_range();
        let hours = date_range.hours();
        assert_eq!(hours, 48)
    }

    #[test]
    fn minutes_test() {
        let date_range = get_date_range();
        let minutes = date_range.minutes();
        assert_eq!(minutes, 2880)
    }

    fn get_date_range() -> DateRange<Date<Local>> {
        DateRange {
            start: Local.ymd(2022, 1, 1),
            end: Local.ymd(2022, 1, 3),
        }
    }
}
use chrono::Duration;

/** 一个连续的时间区间,可以解析其中包含的日，时分等。
 */
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DateRange<T>
where
    T: std::ops::Sub<Output = Duration>,
{
    /// 时间区间的开始
    pub start: T,
    /// 时间区间的结束
    pub end: T,
}

impl<T> DateRange<T>
where
    T: std::ops::Sub<Output = Duration>,
{
    fn get_duration(self) -> Duration {
        self.end - self.start
    }
    /// 返回时间区间内的天数
    pub fn days(self) -> i64 {
        let duration = self.get_duration();
        duration.num_days()
    }
    /// 返回时间区间内的小时数
    pub fn hours(self) -> i64 {
        let duration = self.get_duration();
        duration.num_hours()
    }
    /// 返回时间区间内的分钟数
    pub fn minutes(self) -> i64 {
        let duration = self.get_duration();
        duration.num_minutes()
    }
}
