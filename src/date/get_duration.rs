use chrono::Duration;

pub fn get_duration<T>(date_range: (T, T)) -> Duration
where
    T: std::ops::Sub<Output = Duration>,
{
    let (start, end) = date_range;
    end - start
}
