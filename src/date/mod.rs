use std::time::SystemTime; // Used for SystemTime::now() and SystemTime::UNIX_EPOCK

/// Returns the current year based on an estimate using the seconds since 1970
/// Panics if system time is set to before the unix epoch (January 1, 1970)
/// Currently is naive about how many seconds are in a year
pub fn current_year() -> u32 {
    let seconds_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("System time before January 1, 1970!"); // Seconds since Jan 1 1970 according to system clock

    let seconds_per_minute = 60;
    let seconds_per_hour = 60 * seconds_per_minute;
    let seconds_per_day = 24 * seconds_per_hour;
    let seconds_per_year = 365 * seconds_per_day;

    1970 + (seconds_since_epoch.as_secs() / seconds_per_year) as u32 // Do a rough estimate of the current year
}

/// Quick-n-dirty function to find if a day is between two dates
/// It has no knowledge of how long each month lasts, only that a day must fall on or after the day
/// of the first month and on or before the date of the last month. The order of the arguments
/// matters.
pub fn date_between(month: u32, day: u32, month1: u32, day1: u32, month2: u32, day2: u32) -> bool {
    (month == month1 && day >= day1) || (month == month2 && day <= day2)
}
