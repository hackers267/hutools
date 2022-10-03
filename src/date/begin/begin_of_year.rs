use chrono::{Date, DateTime, Datelike, TimeZone};
#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_year_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 1, 1);
        let result = begin_of_year(date);
        assert_eq!(Some(actual), result);
    }

    #[test]
    fn begin_of_year_with_time_test() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 1, 1);
        let result = begin_of_year_with_time(datetime);
        assert_eq!(Some(actual), result);
    }
}
/// Get the start of a year.
/// 获取某年的开始日期
/// # Arguments
///
/// * `date`: 日期
///
/// returns: Option<Date<Tz>>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_year;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,1,1);
/// let result = begin_of_year(date);
/// assert_eq!(Some(actual), result);
/// ```
pub fn begin_of_year<Tz: TimeZone>(date: Date<Tz>) -> Option<Date<Tz>> {
    date.with_month(1)?.with_day(1)
}

/// Get the start date of a year.
/// 获取某年的开始日期
/// # Arguments
///
/// * `datetime`:
///
/// returns: Option<Date<Tz>>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_year_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,1,1);
/// let result = begin_of_year_with_time(datetime);
/// assert_eq!(Some(actual),result);
/// ```
pub fn begin_of_year_with_time<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<Date<Tz>> {
    begin_of_year(datetime.date())
}
