use plotto_utils::model::Card;

use crate::utils::coersion::bits_to_card;

include!("../generated_hands.rs");

/**
 * Inputs:
 * - a deck (u64 bitmask). This includes any cards that could still be played in the pile.
 * - pile (u64 bitmask)
 *
 * Returns the best 5 card hand possible that includes the pile using cards from the deck.
 */
pub fn arbitrate_pile(deck: u64, pile: u64) -> Option<[Card; 5]> {
    // we can make a hand if:
    // (deck | pile) & hand == hand AND (the hand can be made from the deck and pile)
    // pile & hand == pile              (the hand already includes the pile)
    for hand in SORTED_HANDS.iter() {
        if (deck | pile) & hand == *hand && (pile & hand) == pile {
            return Some(hand_to_cards(*hand));
        }
    }

    None
}

fn hand_to_cards(mut hand: u64) -> [Card; 5] {
    let mut cards: Vec<Card> = Vec::with_capacity(5);

    for _ in 0..5 {
        let bit = 1u64 << hand.trailing_zeros(); // isolate the lowest set bit
        cards.push(bits_to_card(bit));
        hand &= !bit; // clear the bit from the original mask
    }

    return cards
        .try_into()
        .expect("Hand does not contain exactly 5 cards");
}
