use chrono::{Date, NaiveDate, ParseResult, TimeZone, Utc};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, TimeZone};

    #[test]
    fn age_test() {
        let birthday = Local.ymd(2000, 1, 1);
        let date = Local.ymd(2022, 2, 1);
        let age = age(birthday, date);
        assert_eq!(age, Some(22));
    }

    #[test]
    fn age_from_string_test() {
        let birthday = "2000-01-01";
        let date = "2022-02-01";
        let age = age_from_string(birthday, date, None);
        assert_eq!(age, Some(22));
    }
    #[test]
    fn date_from_str_test() {
        let date = "2008-08-08";
        let date = date_utc_from_str(date, None);
        assert_eq!(date, Ok(Utc.ymd(2008, 8, 8)));
    }
    #[test]
    fn date_from_str_error() {
        let date = "2008-08-08 08:08:08";
        let date = date_utc_from_str(date, None);
        assert!(date.is_err())
    }
    #[test]
    fn date_from_str_with_time() {
        let date = "2008-08-08 08:08:08";
        let date = date_utc_from_str(date, Some("%F %T"));
        assert_eq!(date, Ok(Utc.ymd(2008, 8, 8)));
    }
}

///
/// 计算截止到今天的年龄
/// # Arguments
///
/// * `birthday`: 生日
///
/// returns: Option<u32>
///
/// - Some(u32) 当前年龄
/// - None 生日比当前要晚
///
pub fn age_of_now<Tz: TimeZone>(birthday: Date<Tz>) -> Option<u32> {
    let today = Utc::today();
    let birthday = birthday.with_timezone(&Utc);
    age(birthday, today)
}

///
/// 计算到某天的年龄
/// # Arguments
///
/// * `birthday`: 生日
/// * `date`: 截止日期
///
/// returns: Option<u32>
///
/// - Some(u32):    到截止日的年龄
/// - None： 到截止日还没有出生
///
/// # Examples
///
/// ```
/// use chrono::{Local, TimeZone};
/// use hutools::date::age;
/// let birthday = Local.ymd(2020,1,1);
/// let date = Local.ymd(2022,3,2);
/// let age = age(birthday,date);
/// assert_eq!(age,Some(2));
/// ```
pub fn age<Tz: TimeZone>(birthday: Date<Tz>, date: Date<Tz>) -> Option<u32> {
    date.years_since(birthday)
}

///
/// 计算年龄
/// # Arguments
///
/// * `birthday`: 生日
/// * `date`: 日期
///
/// returns: Option<u32>
///
/// # Examples
///
/// ```
/// use hutools::date::age_from_string;
/// let birthday = "2000-01-01";
/// let date = "2012-02-01";
/// let age = age_from_string(birthday,date,None);
/// assert_eq!(age,Some(12));
/// ```
pub fn age_from_string(birthday: &str, date: &str, formatter: Option<&str>) -> Option<u32> {
    let formatter = formatter.unwrap_or("%F");
    let birthday = date_utc_from_str(birthday, None).expect("Invalid date");
    let date_naive = NaiveDate::parse_from_str(date, formatter).unwrap();
    let date = Utc.from_utc_date(&date_naive);
    age(birthday, date)
}

///
/// 从字符串转换为Date<Utc>
/// # Arguments
///
/// * `s`: 字符串
/// * `formatter`: 格式化标记，如 Some(`%Y-%m-%d`),如果是None,则使用默认值 `%Y-%m-%d`
///
/// returns: Result<Date<Utc>, ParseError>
///
fn date_utc_from_str(s: &str, formatter: Option<&str>) -> ParseResult<Date<Utc>> {
    let formatter = formatter.unwrap_or("%F");
    let naive_date = NaiveDate::parse_from_str(s, formatter)?;
    let date = Utc.from_utc_date(&naive_date);
    Ok(date)
}
