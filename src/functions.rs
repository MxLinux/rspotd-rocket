use chrono::format::strftime;
use chrono::{DateTime, Utc};

pub fn get_date_as_ymd() -> String {
    let date: DateTime<Utc> = Utc::now();
    return date.format("%Y-%m-%d").to_string()
}