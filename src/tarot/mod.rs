use std::fmt;
use super::zodiac::Zodiac;

pub enum RiderWaite {
    Fool,
    MajorArcana(u32),
    MinorArcana(Suit, u32),
}

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

impl fmt::Display for RiderWaite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_rep = match &self {
            RiderWaite::Fool => "0. THE FOOL".to_owned(),
            RiderWaite::MajorArcana(n) => match n {
                1  => "I. THE MAGICIAN",
                2  => "II. THE HIGH PRIESTESS",
                3  => "III. THE EMPRESS",
                4  => "IV. THE EMPORER",
                5  => "V. THE HIEROPHANT",
                6  => "VI. THE LOVERS",
                7  => "VII. THE CHARIOT",
                8  => "VIII. STRENGTH",
                9  => "XI. THE HERMIT",
                10 => "X. WHEEL OF FORTUNE",
                11 => "XI. JUSTICE",
                12 => "XII. THE HANGED MAN",
                13 => "XIII. DEATH",
                14 => "XIV. TEMPERANCE",
                15 => "XV. THE DEVIL",
                16 => "XVI. THE TOWER",
                17 => "XVII. THE STAR",
                18 => "XVIII. THE MOON",
                19 => "XIX. THE SUN",
                20 => "XX. THE LAST JUDGEMENT",
                21 => "XXI. THE WORLD",
                _ => "0. THE FOOL",
            }.to_owned(),
            RiderWaite::MinorArcana(suit, number) => {
                let suite_string = match suit {
                    Suit::Wands  => "Wands",
                    Suit::Cups   => "Cups",
                    Suit::Swords => "Swords",
                    Suit::Gold   => "Gold",
                };
                let rank = match number {
                    _ => "22",
                };
                format!("{} OF {}", rank, suite_string)
            },
        };

        write!(f, "{}", string_rep)
    }
}
