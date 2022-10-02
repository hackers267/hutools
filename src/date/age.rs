use chrono::{Date, TimeZone, Utc};

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
