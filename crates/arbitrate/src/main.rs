use crate::utils::arbitrate_pile::arbitrate_pile;
use rand::Rng;
use std::time::Instant;

mod utils;

fn main() {
    let mut rng = rand::thread_rng();
    let mut deck: u64 = rng.gen_range(0..((1 << 53) - 1));
    let mut pile: u64 = 0;

    let num_cards = rng.gen_range(0..5);
    for _ in 0..num_cards {
        let card: u64 = 1 << rng.gen_range(0..52);
        pile |= card;
        deck &= !card;
    }

    let start = Instant::now();
    let best_hand = arbitrate_pile(deck, pile);
    let duration = start.elapsed();
    let start2 = Instant::now();
    arbitrate_pile(deck, pile);
    let duration2 = start2.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Time elapsed: {:?}", duration2);

    println!("Deck: {:052b}", deck);
    println!("Pile: {:052b}", pile);
    match best_hand {
        Some(hand) => {
            println!("Best hand found:");
            for card in hand.iter() {
                println!("{:?} of {:?}", card.rank, card.suit);
            }
        }
        None => println!("No valid hand could be formed with the given pile and deck."),
    }
}
