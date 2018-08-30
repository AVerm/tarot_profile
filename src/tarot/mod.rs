use super::zodiac::Zodiac;

#[derive(Debug)]
pub enum RiderWaite {
    Fool,
    MajorArcana(u32),
    MinorArcana(Suit, u32),
}

#[derive(Debug)]
pub enum Suit {
    Wands,
    Cups,
    Swords,
    Gold,
}

/// Sourced from here: https://www.tarot.com/astrology/tarot-cards
pub fn zodiac_card(zodiac_sign: Zodiac) -> RiderWaite {
    match zodiac_sign {
        Zodiac::Aries       => RiderWaite::MajorArcana(4),  // Emporer
        Zodiac::Taurus      => RiderWaite::MajorArcana(5),  // Hierophant
        Zodiac::Gemini      => RiderWaite::MajorArcana(5),  // Lover
        Zodiac::Cancer      => RiderWaite::MajorArcana(7),  // Chariot
        Zodiac::Leo         => RiderWaite::MajorArcana(8),  // Strength
        Zodiac::Virgo       => RiderWaite::MajorArcana(9),  // Hermit
        Zodiac::Libra       => RiderWaite::MajorArcana(11), // Justice
        Zodiac::Scorpio     => RiderWaite::MajorArcana(13), // Death
        Zodiac::Sagittarius => RiderWaite::MajorArcana(14), // Temperance
        Zodiac::Capricorn   => RiderWaite::MajorArcana(15), // Devil
        Zodiac::Aquarius    => RiderWaite::MajorArcana(17), // Star
        Zodiac::Pisces      => RiderWaite::MajorArcana(18), // Moon
    }
}

// TODO: Write printing for the cards
