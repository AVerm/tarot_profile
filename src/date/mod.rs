use std::time::SystemTime; // Used for SystemTime::now() and SystemTime::UNIX_EPOCK

/// Returns the current year based on an estimate using the seconds since 1970
/// Panics if system time is set to before the unix epoch (January 1, 1970)
/// Currently is naive about how many seconds are in a year
pub fn current_year() -> u32 {
    let seconds_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("System time before January 1, 1970!");

    let seconds_per_minute = 60;
    let seconds_per_hour = 60 * seconds_per_minute;
    let seconds_per_day = 24 * seconds_per_hour;
    let seconds_per_year = 365 * seconds_per_day;

    let year = 1970 + (seconds_since_epoch.as_secs() / seconds_per_year) as u32;

    year
}

pub fn date_between(month: u32, day: u32, month1: u32, day1: u32, month2: u32, day2: u32) -> bool {
    (month == month1 && day >= day1) || (month == month2 && day <= day2)
}
