use chrono::{Datelike, Utc};

// get current year in strin
pub fn current_year_string() -> String {
    Utc::now().year().to_string()
}

// days wasted by the user
pub fn days_passed() -> (u32, u32) {
    let days_passed_actually = Utc::now().ordinal() - 1;
    (days_passed_actually, ((days_passed_actually as f32 / days_left() as f32) * 100.0).round() as u32)
}

// days still have for user to waste
pub fn days_left() -> u32 {
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    if is_leap_year(Utc::now().year()) { 366 } else { 365 }
}

