use date::date_between;

/// Sourced from here: https://www.tarot.com/astrology/tarot-cards
pub fn zodiac_sign(month: u32, day: u32) -> Zodiac {
    return
         if date_between(month, day, 03, 21, 04, 19) {Zodiac::Aries}
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

#[derive(Debug)]
pub enum Zodiac {
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
