use chrono::{Datelike, Utc};

// get current year in strin
pub fn current_year_string() -> i32 {
    Utc::now().year()
}

// days wasted by the user
pub fn days_passed() -> (i32, i32) {
    let days_passed_actually: i32 = Utc::now().ordinal() as i32 - 1;
    (days_passed_actually, ((days_passed_actually as f32 / days_in_year() as f32) * 100.0).round() as i32)
}

// days still have for user to waste
pub fn days_in_year() -> i32 {
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    if is_leap_year(Utc::now().year()) { 366 } else { 365 }
}

