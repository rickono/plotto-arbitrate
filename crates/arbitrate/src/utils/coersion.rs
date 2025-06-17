use plotto_utils::model::{Card, Rank, Suit};

pub fn bits_to_card(bits: u64) -> Card {
    let bit_index = bits.trailing_zeros() as u8;
    let rank_val = bit_index / 4;
    let suit_val = bit_index % 4;
    return Card {
        rank: Rank::try_from(rank_val).expect("Invalid rank"),
        suit: Suit::try_from(suit_val).expect("Invalid suit"),
    };
}
