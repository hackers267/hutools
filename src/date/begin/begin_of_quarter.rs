use chrono::{Date, DateTime, Datelike, TimeZone};
#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn begin_of_quarter_test() {
        let date = Utc.ymd(2008, 8, 8);
        let actual = Utc.ymd(2008, 6, 1);
        let result = begin_of_quarter(date);
        assert_eq!(Some(actual), result);
    }
    #[test]
    fn begin_of_first_quarter_test() {
        let date = Utc.ymd(2008, 2, 2);
        let actual = Utc.ymd(2007, 12, 1);
        let result = begin_of_quarter(date);
        assert_eq!(Some(actual), result);
    }
}

/// Get the start date of the quarter.
/// 获取某个季度的开始日期
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
/// use hutools::date::begin_of_quarter;
/// let date = Utc.ymd(2008,8,8);
/// let actual = Utc.ymd(2008,6,1);
/// let result = begin_of_quarter(date);
/// assert_eq!(Some(actual), result);
/// ```
pub fn begin_of_quarter<Tz: TimeZone>(date: Date<Tz>) -> Option<Date<Tz>> {
    let month = date.month();
    let year = date.year();
    if month == 12 || month == 1 || month == 2 {
        return date.with_year(year - 1)?.with_month(12)?.with_day(1);
    }
    if month == 3 || month == 4 || month == 5 {
        return date.with_month(3)?.with_day(1);
    }
    if month == 6 || month == 7 || month == 8 {
        return date.with_month(6)?.with_day(1);
    }
    if month == 9 || month == 10 || month == 11 {
        return date.with_month(9)?.with_day(1);
    }
    None
}

/// Get the start date of a quarter.
/// 获取某个季度的开始日期
/// # Arguments
///
/// * `datetime`: 日期时间
///
/// returns: Option<Date<Tz>>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_quarter_with_time;
/// let datetime = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,6,1);
/// let result = begin_of_quarter_with_time(datetime);
/// assert_eq!(Some(actual), result);
/// ```
pub fn begin_of_quarter_with_time<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<Date<Tz>> {
    let month = datetime.month();
    if month == 12 || month == 1 || month == 2 {
        let year = datetime.year();
        return datetime
            .date()
            .with_year(year - 1)?
            .with_month(12)?
            .with_day(1);
    }
    if month == 3 || month == 4 || month == 5 {
        return datetime.date().with_month(3)?.with_day(1);
    }
    if month == 6 || month == 7 || month == 8 {
        return datetime.date().with_month(6)?.with_day(1);
    }
    if month == 9 || month == 10 || month == 11 {
        return datetime.date().with_month(9)?.with_day(1);
    }
    None
}
