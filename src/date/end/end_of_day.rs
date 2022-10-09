use chrono::{DateTime, TimeZone};

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_end_of_day() {
        let datetime = Utc.ymd(2008, 8, 8).and_hms(8, 8, 8);
        let result = end_of_day(datetime);
        let actual = Utc.ymd(2008, 8, 8).and_hms(23, 59, 59);
        assert_eq!(result, actual);
    }
}

pub fn end_of_day<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    datetime.date().and_hms(23, 59, 59)
}
