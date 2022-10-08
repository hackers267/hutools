use chrono::{Date, DateTime, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_between_years_with_time() {
        let start = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let end = Utc.ymd(2018, 8, 8).and_hms(0, 0, 0);
        let date_range = (start, end);
        let actual = 9;
        let result = between_years_with_time(date_range);
        assert_eq!(result, Some(actual));
    }

    #[test]
    fn test_between_years() {
        let start = Utc.ymd(2008, 8, 8);
        let end = Utc.ymd(2018, 8, 8);
        let date_range = (start, end);
        let actual = 10;
        let result = between_years(date_range);
        assert_eq!(result, Some(actual));
    }

    #[test]
    fn test_between_years_none() {
        let end = Utc.ymd(2008, 8, 8);
        let start = Utc.ymd(2018, 8, 8);
        let date_range = (start, end);
        let result = between_years(date_range);
        assert_eq!(result, None);
    }
}

pub fn between_years<Tz: TimeZone>(date_range: (Date<Tz>, Date<Tz>)) -> Option<i64> {
    let (start, end) = date_range;
    let symbol = if start <= end { 1 } else { -1 };
    end.years_since(start)
        .and_then(|x| (x as i64).checked_mul(symbol))
}

pub fn between_years_with_time<Tz: TimeZone>(
    date_range: (DateTime<Tz>, DateTime<Tz>),
) -> Option<u32> {
    let (start, end) = date_range;
    end.years_since(start)
}
