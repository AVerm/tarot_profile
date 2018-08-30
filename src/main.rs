use std::io; // Needed to use stdin
use std::io::Write; // Needed to flush stdout

pub mod tarot; // Custom module containing Tarot structs and functions
pub mod date;
use date::date_between;

struct Date(u32, u32); // (Month, Day)

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

    let personality_card = tarot::RiderWaite::MajorArcana(personality_number);
    let soul_card        = tarot::RiderWaite::MajorArcana(soul_number);
    let year_card        = tarot::RiderWaite::MajorArcana(year_number);
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

/// Sourced from here: https://www.tarot.com/astrology/tarot-cards
fn zodiac_sign(month: u32, day: u32) -> Zodiac {
    return
    if      date_between(month, day, 03, 21, 04, 19) {Zodiac::Aries}
    else if date_between(month, day, 02, 20, 05, 20) {Zodiac::Taurus}
    else if date_between(month, day, 05, 21, 06, 20) {Zodiac::Gemini}
    else if date_between(month, day, 06, 21, 07, 22) {Zodiac::Cancer}
    else if date_between(month, day, 07, 23, 08, 22) {Zodiac::Leo}
    else if date_between(month, day, 08, 23, 09, 22) {Zodiac::Virgo}
    else if date_between(month, day, 09, 23, 10, 22) {Zodiac::Libra}
    else if date_between(month, day, 10, 23, 11, 21) {Zodiac::Scorpio}
    else if date_between(month, day, 10, 22, 12, 21) {Zodiac::Sagittarius}
    else if date_between(month, day, 12, 22, 01, 19) {Zodiac::Capricorn}
    else if date_between(month, day, 01, 20, 02, 18) {Zodiac::Aquarius}
    else if date_between(month, day, 02, 19, 03, 20) {Zodiac::Pisces}
    else { panic!("Not a valid date!");}
}

enum Zodiac {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}
