use chrono::{Date, Datelike, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_between_months() {
        let start = Utc.ymd(2008, 8, 8);
        let end = Utc.ymd(2018, 9, 9);
        let date_range = (start, end);
        let result = between_month(date_range);
        let actual = 121;
        assert_eq!(result, Some(actual));
    }

    #[test]
    fn test_between_month_1() {
        let start = Utc.ymd(2008, 8, 8);
        let end = Utc.ymd(2018, 9, 1);
        let data_range = (start, end);
        let result = between_month(data_range);
        let actual = 120;
        assert_eq!(result, Some(actual));
    }
    #[test]
    fn test_between_months_2() {
        let start = Utc.ymd(2008, 8, 8);
        let end = Utc.ymd(2018, 8, 16);
        let date_range = (start, end);
        let result = between_month(date_range);
        let actual = 120;
        assert_eq!(result, Some(actual));
    }
    #[test]
    fn test_between_months_3() {
        let start = Utc.ymd(2008, 8, 8);
        let end = Utc.ymd(2018, 8, 2);
        let date_range = (start, end);
        let result = between_month(date_range);
        let actual = 119;
        assert_eq!(result, Some(actual));
    }
}

pub fn between_month<Tz>(date_range: (Date<Tz>, Date<Tz>)) -> Option<i32>
where
    Tz: TimeZone,
{
    let (start, end) = date_range;
    let start_year = start.year();
    let end_year = end.year();
    let start_month = start.month() as i32;
    let end_month = end.month() as i32;
    let start_day = start.day();
    let end_day = end.day();
    let diff_day = end_day.checked_sub(start_day);
    let diff = if diff_day.is_some() { 0 } else { -1 };
    let diff_month = end_month
        .checked_sub(start_month)
        .and_then(|x| x.checked_add(diff));
    let diff_year = end_year.checked_sub(start_year);
    diff_year
        .and_then(|x| x.checked_mul(12))
        .and_then(|x| diff_month.and_then(|y| y.checked_add(x)))
}
