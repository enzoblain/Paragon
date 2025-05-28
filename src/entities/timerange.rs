pub struct Timerange {
    pub label: &'static str,
    pub duration_ms: u128, // Corresponding durations in milliseconds for the timeranges
}

pub static TIMERANGES: &[Timerange] = &[
    Timerange { label: "1min", duration_ms: 60_000 },
    Timerange { label: "5min", duration_ms: 300_000 },
    Timerange { label: "15min", duration_ms: 900_000 },
    Timerange { label: "30min", duration_ms: 1_800_000 },
    Timerange { label: "1h", duration_ms: 3_600_000 },
    Timerange { label: "4h", duration_ms: 14_400_000 },
    Timerange { label: "1d", duration_ms: 86_400_000 },
    Timerange { label: "1w", duration_ms: 604_800_000 },
];