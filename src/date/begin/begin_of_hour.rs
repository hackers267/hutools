use chrono::{DateTime, TimeZone, Timelike};
#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    #[test]
    fn begin_of_hour_test() {
        let time = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let actual = Utc.ymd(2008, 8, 8).and_hms(8, 0, 0);
        let result = begin_of_hour(time);
        assert_eq!(actual, result);
    }
}

/// Get the begin of hour
/// 获取某小时的开始时间
/// # Arguments
///
/// * `date_time`: the date time （时间）
///
/// returns: DateTime<Tz>
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use hutools::date::begin_of_hour;
/// let time = Utc.ymd(2008,8,8).and_hms(8,8,8);
/// let actual = Utc.ymd(2008,8,8).and_hms(8,0,0);
/// let result = begin_of_hour(time);
/// assert_eq!(actual,result);
/// ```
pub fn begin_of_hour<Tz: TimeZone>(date_time: DateTime<Tz>) -> DateTime<Tz> {
    let hour = date_time.hour();
    date_time.date().and_hms(hour, 0, 0)
}
