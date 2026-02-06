use chrono::{Datelike, Utc};

pub fn current_year_string() -> String {
    Utc::now().year().to_string()
}
