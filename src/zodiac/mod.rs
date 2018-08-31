use date::date_between;

/// Sourced from here: https://www.tarot.com/astrology/tarot-cards
pub fn zodiac_sign(month: u32, day: u32) -> Zodiac {
         if date_between(month, day,  3, 21,  4, 19) {Zodiac::Aries}
    else if date_between(month, day,  2, 20,  5, 20) {Zodiac::Taurus}
    else if date_between(month, day,  5, 21,  6, 20) {Zodiac::Gemini}
    else if date_between(month, day,  6, 21,  7, 22) {Zodiac::Cancer}
    else if date_between(month, day,  7, 23,  8, 22) {Zodiac::Leo}
    else if date_between(month, day,  8, 23,  9, 22) {Zodiac::Virgo}
    else if date_between(month, day,  9, 23, 10, 22) {Zodiac::Libra}
    else if date_between(month, day, 10, 23, 11, 21) {Zodiac::Scorpio}
    else if date_between(month, day, 10, 22, 12, 21) {Zodiac::Sagittarius}
    else if date_between(month, day, 12, 22,  1, 19) {Zodiac::Capricorn}
    else if date_between(month, day,  1, 20,  2, 18) {Zodiac::Aquarius}
    else if date_between(month, day,  2, 19,  3, 20) {Zodiac::Pisces}
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
