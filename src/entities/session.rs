use chrono::{
    DateTime,
    NaiveTime,
    Utc
};

#[derive(Clone)]
pub struct Session {
    pub label: &'static str,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

pub struct ReferenceSession {
    pub label: &'static str,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

// All the sessions are in UTC time
pub static SESSIONS: &[ReferenceSession] = &[
    ReferenceSession {
        label: "Asian Session",
        start: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
        end: NaiveTime::from_hms_opt(7, 30, 0).unwrap(),
    },
    ReferenceSession {
        label: "London Session",
        start: NaiveTime::from_hms_opt(7, 30, 0).unwrap(),
        end: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    },
    ReferenceSession {
        label: "New York Session",
        start: NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        end: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
    },
];