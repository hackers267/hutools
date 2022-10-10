use chrono::{Date, DateTime, Datelike, TimeZone};
use chronoutil::is_leap_year;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_end_of_month() {
        let date = Utc.ymd(2008, 8, 8);
        let result = end_of_month(date);
        let actual = Utc.ymd(2008, 8, 31);
        assert_eq!(result, actual);
    }
    #[test]
    fn test_end_of_month_4() {
        let date = Utc.ymd(2008, 4, 2);
        let result = end_of_month(date);
        let actual = Utc.ymd(2008, 4, 30);
        assert_eq!(result, actual);
    }
    #[test]
    fn end_of_month_2008_2() {
        let date = Utc.ymd(2008, 2, 2);
        let result = end_of_month(date);
        let actual = Utc.ymd(2008, 2, 29);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_end_of_month_2007_2() {
        let date = Utc.ymd(2007, 2, 2);
        let result = end_of_month(date);
        let actual = Utc.ymd(2007, 2, 28);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_end_of_month_with_time() {
        let datetime = Utc.ymd(2008, 2, 2).and_hms(8, 8, 8);
        let result = end_of_month_with_time(datetime);
        let actual = Utc.ymd(2008, 2, 29).and_hms(23, 59, 59);
        assert_eq!(result, actual);
    }
}

/// Get the end of month;
/// 获取某个月的结束日期
/// # Arguments
///
/// * `date`:
///
/// returns: Date<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::end_of_month;
/// let date = Utc.ymd(2008,2,2);
/// let result = end_of_month(date);
/// let actual = Utc.ymd(2008,2,29);
/// assert_eq!(actual, result);
/// ```
pub fn end_of_month<Tz>(date: Date<Tz>) -> Date<Tz>
where
    Tz: TimeZone,
{
    let month = date.month();
    let day31 = [1, 3, 5, 7, 8, 10, 12];
    let day30 = [4, 6, 9, 10];
    if day31.contains(&(month as i32)) {
        return date.with_day(31).unwrap();
    }
    if day30.contains(&(month as i32)) {
        return date.with_day(30).unwrap();
    }
    let year = date.year();
    let leap_year = is_leap_year(year);
    if leap_year {
        return date.with_day(29).unwrap();
    }
    date.with_day(28).unwrap()
}

/// Get the end time of month.
///
/// # Arguments
///
/// * `datetime`:
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::end_of_month_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let result = end_of_month_with_time(datetime);
/// let actual = Utc.ymd(2008,8,31).and_hms(23,59,59);
/// assert_eq!(result, actual);
/// ```
pub fn end_of_month_with_time<Tz>(datetime: DateTime<Tz>) -> DateTime<Tz>
where
    Tz: TimeZone,
{
    let date = end_of_month(datetime.date());
    date.and_hms(23, 59, 59)
}
