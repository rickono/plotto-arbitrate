use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
