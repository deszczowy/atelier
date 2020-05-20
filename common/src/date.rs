use chrono::{Utc};

pub fn time_stamp() -> String {
    Utc::now().format("%Y%m%d%H%M%S%3f").to_string()
}