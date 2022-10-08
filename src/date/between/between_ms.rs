use crate::date::get_duration::get_duration;
use chrono::Duration;
use std::ops::Sub;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_between_ms() {
        let start = Utc.ymd(2008, 8, 8).and_hms_micro(8, 8, 8, 1);
        let end = Utc.ymd(2008, 8, 8).and_hms_micro(8, 8, 8, 999);
        let date_range = (start, end);
        let result = between_ms(date_range);
        let actual = 998;
        assert_eq!(result, Some(actual));
    }
}

pub fn between_ms<T>(date_range: (T, T)) -> Option<i64>
where
    T: Sub<Output = Duration>,
{
    let duration = get_duration(date_range);
    duration.num_microseconds()
}
