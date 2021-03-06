use std::io; // Needed to use stdin
use std::io::Write; // Needed to flush stdout

mod date;
mod tarot;
mod zodiac;

use tarot::major_arcana_num;
use zodiac::zodiac_sign;

fn main() {
    println!("Please enter your");

    let year:  u32 = get_input("\tBirth year (as a number): ");
    let month: u32 = get_input("\tBirth month (as a number): ");
    let day:   u32 = get_input("\tBirth day (as a number): ");

    let personality_number = digit_sum(year + month + day);
    let soul_number = digit_sum(personality_number);
    let year_number = digit_sum(date::current_year() + month + day);
    let zodiac_sign = zodiac_sign(month, day);

    let personality_card = major_arcana_num(personality_number);
    let soul_card        = major_arcana_num(soul_number);
    let year_card        = major_arcana_num(year_number);
    let zodiac_card      = tarot::zodiac_card(&zodiac_sign);

    println!();
    println!("Personality card:\t{}", personality_card);
    println!("Soul card:\t\t{}", soul_card);
    println!("Year card:\t\t{}", year_card);
    println!("Zodiac card:\t\t{}", zodiac_card);
}

/// Given an instruction string, prints the string and reads input from the command line,
/// attempting to cast from a string to the target type. Loops until valid output is possible.
fn get_input<T, E>(instructions: &str) -> T
where T: Sized + std::str::FromStr<Err=E>,
      E: std::fmt::Debug,
{
    let mut input = "".to_string();
    let mut num: Result<T, <T as std::str::FromStr>::Err>;

    while { // This loop uses a dirty hack to emulate a do-while loop. Details at https://gist.github.com/huonw/8435502
        print!("{}", instructions);
        io::stdout().flush().expect("Could not flush stdout");
        input = "".to_string();
        let _ = io::stdin().read_line(&mut input);
        num = input.trim_right().parse::<T>();
        num.is_err()
    } {}

    num.expect("Loop in get_input exited with error state, should not be possible")
}

/// Utility function that sums the digits of a number
/// ```
/// assert_eq!(digit_sum(1234), 10);
/// ```
fn digit_sum(mut num: u32) -> u32 {
    let mut remainder;
    let mut sum = 0;
    while num != 0
    {
        remainder = num % 10;
        sum += remainder;
        num /= 10;
    }
    sum
}
