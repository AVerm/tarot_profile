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

// TODO: Write printing for the cards
