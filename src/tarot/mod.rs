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

// TODO: Write printing for the cards
