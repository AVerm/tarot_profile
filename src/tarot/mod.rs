use std::fmt;
use super::zodiac::Zodiac;

pub enum RiderWaite {
    MajorArcana(MajorArcana),
    MinorArcana(Suit, Rank),
}

pub enum MajorArcana {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emporer,
    Hierophant,
    Lovers,
    Chariot,
    Strength,
    Hermit,
    WheelOfFortune,
    Justice,
    HangedMan,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    LastJudgement,
    World,
}

pub enum Suit {
    Wands,
    Cups,
    Swords,
    Gold,
}

pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

/// Sourced from here: https://www.tarot.com/astrology/tarot-cards
pub fn zodiac_card(zodiac_sign: Zodiac) -> RiderWaite {
    RiderWaite::MajorArcana (
        match zodiac_sign {
            Zodiac::Aries       => MajorArcana::Emporer,
            Zodiac::Taurus      => MajorArcana::Hierophant,
            Zodiac::Gemini      => MajorArcana::Lovers,
            Zodiac::Cancer      => MajorArcana::Chariot,
            Zodiac::Leo         => MajorArcana::Strength,
            Zodiac::Virgo       => MajorArcana::Hermit,
            Zodiac::Libra       => MajorArcana::Justice,
            Zodiac::Scorpio     => MajorArcana::Death,
            Zodiac::Sagittarius => MajorArcana::Temperance,
            Zodiac::Capricorn   => MajorArcana::Devil,
            Zodiac::Aquarius    => MajorArcana::Star,
            Zodiac::Pisces      => MajorArcana::Moon,
        }
    )
}

pub fn major_arcana_num(num: u32) -> RiderWaite {
    RiderWaite::MajorArcana (
        match num {
            1  => MajorArcana::Magician,
            2  => MajorArcana::HighPriestess,
            3  => MajorArcana::Empress,
            4  => MajorArcana::Emporer,
            5  => MajorArcana::Hierophant,
            6  => MajorArcana::Lovers,
            7  => MajorArcana::Chariot,
            8  => MajorArcana::Strength,
            9  => MajorArcana::Hermit,
            10 => MajorArcana::WheelOfFortune,
            11 => MajorArcana::Justice,
            12 => MajorArcana::HangedMan,
            13 => MajorArcana::Death,
            14 => MajorArcana::Temperance,
            15 => MajorArcana::Devil,
            16 => MajorArcana::Tower,
            17 => MajorArcana::Star,
            18 => MajorArcana::Moon,
            19 => MajorArcana::Sun,
            20 => MajorArcana::LastJudgement,
            21 => MajorArcana::World,
            _  => MajorArcana::Fool,
        }
    )
}

impl fmt::Display for RiderWaite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_rep = match &self {
            RiderWaite::MajorArcana(major_arcana) => format!("{}", major_arcana),
            RiderWaite::MinorArcana(suit, rank) => format!("{} OF {}", rank, suit),
        };

        write!(f, "{}", string_rep)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Suit::Wands  => "WANDS",
                   Suit::Cups   => "CUPS",
                   Suit::Swords => "SWORDS",
                   Suit::Gold   => "GOLD",
               }
        )
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Rank::One    => "ONE",
                   Rank::Two    => "TWO",
                   Rank::Three  => "THREE",
                   Rank::Four   => "FOUR",
                   Rank::Five   => "FIVE",
                   Rank::Six    => "SIX",
                   Rank::Seven  => "SEVEN",
                   Rank::Eight  => "EIGHT",
                   Rank::Nine   => "NINE",
                   Rank::Ten    => "TEN",
                   Rank::Page   => "PAGE",
                   Rank::Knight => "KNIGHT",
                   Rank::Queen  => "QUEEN",
                   Rank::King   => "KING",
               }
        )
    }
}

impl fmt::Display for MajorArcana {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   MajorArcana::Fool           => "0. THE FOOL",
                   MajorArcana::Magician       => "I. THE MAGICIAN",
                   MajorArcana::HighPriestess  => "II. THE HIGH PRIESTESS",
                   MajorArcana::Empress        => "III. THE EMPRESS",
                   MajorArcana::Emporer        => "IV. THE EMPORER",
                   MajorArcana::Hierophant     => "V. THE HIEROPHANT",
                   MajorArcana::Lovers         => "VI. THE LOVERS",
                   MajorArcana::Chariot        => "VII. THE CHARIOT",
                   MajorArcana::Strength       => "VIII. STRENGTH",
                   MajorArcana::Hermit         => "XI. THE HERMIT",
                   MajorArcana::WheelOfFortune => "X. WHEEL OF FORTUNE",
                   MajorArcana::Justice        => "XI. JUSTICE",
                   MajorArcana::HangedMan      => "XII. THE HANGED MAN",
                   MajorArcana::Death          => "XIII. DEATH",
                   MajorArcana::Temperance     => "XIV. TEMPERANCE",
                   MajorArcana::Devil          => "XV. THE DEVIL",
                   MajorArcana::Tower          => "XVI. THE TOWER",
                   MajorArcana::Star           => "XVII. THE STAR",
                   MajorArcana::Moon           => "XVIII. THE MOON",
                   MajorArcana::Sun            => "XIX. THE SUN",
                   MajorArcana::LastJudgement  => "XX. THE LAST JUDGEMENT",
                   MajorArcana::World          => "XXI. THE WORLD",
               }
        )
    }
}
