#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Date, Local, TimeZone};

    #[test]
    fn days_test() {
        let date_range = get_date_range();
        let days = date_range.days();
        assert_eq!(days, 59)
    }
    #[test]
    fn hours_test() {
        let date_range = get_date_range();
        let hours = date_range.hours();
        assert_eq!(hours, 1416)
    }

    #[test]
    fn minutes_test() {
        let date_range = get_date_range();
        let minutes = date_range.minutes();
        assert_eq!(minutes, 84960)
    }

    fn get_date_range() -> DateRange<Date<Local>> {
        DateRange {
            start: Local.ymd(2022, 1, 1),
            end: Local.ymd(2022, 3, 1),
        }
    }

    #[test]
    fn interval_left_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2000, 1, 1),
            end: Local.ymd(2010, 1, 1),
        };
        let result = date_range.relation(&another_date_range);
        assert_eq!(result, Relation::Left);
    }
    #[test]
    fn interval_left_intersection_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2021, 1, 1),
            end: Local.ymd(2022, 1, 2),
        };
        let relation = date_range.relation(&another_date_range);
        assert_eq!(relation, Relation::LeftIntersection);
    }
    #[test]
    fn interval_include_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2022, 1, 10),
            end: Local.ymd(2022, 2, 10),
        };
        let relation = date_range.relation(&another_date_range);
        assert_eq!(relation, Relation::Include);
    }
    #[test]
    fn interval_right_intersection_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2022, 2, 1),
            end: Local.ymd(2022, 5, 1),
        };
        let relation = date_range.relation(&another_date_range);
        assert_eq!(relation, Relation::RightIntersection);
    }
    #[test]
    fn interval_right_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2023, 1, 1),
            end: Local.ymd(2025, 1, 1),
        };
        let relation = date_range.relation(&another_date_range);
        assert_eq!(relation, Relation::Right);
    }
    #[test]
    fn interval_included_test() {
        let date_range = get_date_range();
        let another_date_range = DateRange {
            start: Local.ymd(2021, 1, 1),
            end: Local.ymd(2025, 1, 1),
        };
        let relation = date_range.relation(&another_date_range);
        assert_eq!(relation, Relation::Included)
    }
}

use crate::interval::{Interval, Relation};
use chrono::Duration;
use std::ops::Sub;

/** ???????????????????????????,?????????????????????????????????????????????
 */
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DateRange<T>
where
    T: Sub<Output = Duration>,
{
    /// ?????????????????????
    pub start: T,
    /// ?????????????????????
    pub end: T,
}

impl<T> DateRange<T>
where
    T: Sub<Output = Duration>,
{
    fn get_duration(self) -> Duration {
        self.end - self.start
    }
    /// ??????????????????????????????
    pub fn days(self) -> i64 {
        let duration = self.get_duration();
        duration.num_days()
    }
    /// ?????????????????????????????????
    pub fn hours(self) -> i64 {
        let duration = self.get_duration();
        duration.num_hours()
    }
    /// ?????????????????????????????????
    pub fn minutes(self) -> i64 {
        let duration = self.get_duration();
        duration.num_minutes()
    }
}

impl<T> Interval for DateRange<T>
where
    T: Sub<Output = Duration> + PartialOrd,
{
    /**
     * ???????????????????????????????????????`Relation`:
     *
     * - `Relation::Left` ???????????????????????????????????????
     * - `Relation::LeftIntersection` ??????????????????????????????????????????????????????
     * - `Relation::Include` ????????????????????????????????????????????????
     * - `Relation::RightIntersection` ?????????????????????????????????????????????????????????
     * - `Relation::Right` ???????????????????????????????????????
     * - `Relation::Included` ????????????????????????????????????????????????
     */
    fn relation(&self, another: &DateRange<T>) -> Relation {
        let DateRange { start, end } = self;
        let DateRange {
            start: other_start,
            end: other_end,
        } = another;
        if other_end < start {
            return Relation::Left;
        }
        if other_start > end {
            return Relation::Right;
        }
        if start < other_start && other_end < end {
            return Relation::Include;
        }
        if other_start < start && end < other_end {
            return Relation::Included;
        }
        if start < other_end && other_end < end {
            Relation::LeftIntersection
        } else {
            Relation::RightIntersection
        }
    }
}
impl<T> DateRange<T>
where
    T: Sub<Output = Duration> + PartialOrd,
{
    /// ??????????????????????????????????????????????????????
    ///
    /// # Arguments
    ///
    /// * `another`: ??????????????????
    ///
    /// returns: Vec<DateRange<T>, Global>
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{Local, TimeZone};
    /// use hutools::date::DateRange;
    /// let date_range = DateRange{start:Local.ymd(2022,5,1),end:Local.ymd(2022,10,1)};
    /// let another_range = DateRange{start:Local.ymd(2022,8,1),end:Local.ymd(2022,12,1)};
    /// let result = date_range.diff(another_range);
    /// assert_eq!(result,vec![DateRange{start:Local.ymd(2022,5,1),end:Local.ymd(2022,8,1)}])
    /// ```
    pub fn diff(self, another: Self) -> Vec<DateRange<T>> {
        let relation = self.relation(&another);
        match relation {
            Relation::Left => vec![self],
            Relation::LeftIntersection => {
                let DateRange { end, start: _ } = self;
                let DateRange {
                    end: start,
                    start: _,
                } = another;
                vec![DateRange { start, end }]
            }
            Relation::Include => {
                let DateRange { start, end } = self;
                let DateRange {
                    start: other_start,
                    end: other_end,
                } = another;
                vec![
                    DateRange {
                        start,
                        end: other_start,
                    },
                    DateRange {
                        start: other_end,
                        end,
                    },
                ]
            }
            Relation::RightIntersection => {
                let DateRange { start, end: _ } = self;
                let DateRange {
                    start: other_start,
                    end: _,
                } = another;
                vec![DateRange {
                    start,
                    end: other_start,
                }]
            }
            Relation::Right => vec![self],
            Relation::Included => vec![],
        }
    }
}
