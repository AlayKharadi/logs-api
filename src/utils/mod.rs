use chrono::{DateTime, Utc};

const BASE_DIR: &str = "./logs";

pub fn partition(dt: &DateTime<Utc>) -> String {
    format!(
        "{}/year={}/month={}/day={}/hour={}/minute={}",
        BASE_DIR,
        dt.format("%Y"),
        dt.format("%m"),
        dt.format("%d"),
        dt.format("%H"),
        dt.format("%M")
    )
}
