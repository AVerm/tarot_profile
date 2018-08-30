use std::io; // Needed to use stdin
use std::io::Write; // Needed to flush stdout

mod date;
mod tarot;
mod zodiac;

use tarot::RiderWaite;
use zodiac::{zodiac_sign, Zodiac};

fn main() {
    println!("Please enter your");

    let year: u32 = get_input("\tBirth year (as a number): ");
    let month: u32 = get_input("\tBirth month (as a number): ");
    let day: u32 = get_input("\tBirth day (as a number): ");

    let year_sum = digit_sum(year);
    let month_sum = digit_sum(month);
    let day_sum = digit_sum(day);

    let personality_number = year_sum + month_sum + day_sum;
    let soul_number = digit_sum(personality_number);
    let year_number = digit_sum(date::current_year()) + month_sum + day_sum;
    let zodiac_sign = zodiac_sign(month, day);

    let personality_card = RiderWaite::MajorArcana(personality_number);
    let soul_card        = RiderWaite::MajorArcana(soul_number);
    let year_card        = RiderWaite::MajorArcana(year_number);
    //let zodiac_card      = tarot::RiderWaite::MajorArcana(z
    
    println!("Personality card: {:?}", personality_card);
    println!("Soul card: {:?}", soul_card);
    println!("Year card: {:?}", year_card);
    //println!("Zodiac card: {}", zodiac_card);
}

fn get_input<T, E>(instructions: &str) -> T
where T: Sized + std::str::FromStr<Err=E>,
      E: std::fmt::Debug,
{
    print!("{}", instructions);
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut input = "".to_owned();
    let mut num: Result<T, <T as std::str::FromStr>::Err>;

    while { // This loop uses a dirty hack to emulate a do-while loop. Details at https://gist.github.com/huonw/8435502
        let _ = io::stdin().read_line(&mut input);
        num = input.trim_right().parse::<T>();
        num.is_err()
    } {}

    num.expect("Loop in get_input exited with error state, should not be possible")
}

fn digit_sum(mut num: u32) -> u32 {
    let mut remainder;
    let mut sum = 0;
    while num != 0
    {
        remainder = num % 10;
        sum = sum + remainder;
        num = num / 10;
    }
    sum
}
