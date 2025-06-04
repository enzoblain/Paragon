use chrono::NaiveTime;

pub fn is_in_timerange(start: NaiveTime, end: NaiveTime, time: NaiveTime) -> bool {
    if start <= end {
        time >= start && time <= end
    } else {
        // If the range wraps around midnight
        time >= start || time <= end
    }
}