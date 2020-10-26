use chrono::{Utc};

pub fn time_stamp() -> String {
    Utc::now().format("%Y%m%d%H%M%S%3f").to_string()
}

pub fn perfect_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}