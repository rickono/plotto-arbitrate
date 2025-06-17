use std::cmp::Ordering;

use crate::model::Rank;

const RANK_MASK: u64 = 0b1111;
const SUIT_MASK: u64 = 0b0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001;

pub fn compare_hands(hand1: u64, hand2: u64) -> Ordering {
    // straight flush
    let hand1_flush = flush(hand1);
    let hand2_flush = flush(hand2);
    let hand1_straight = straight(hand1);
    let hand2_straight = straight(hand2);

    if hand1_flush && hand1_straight.is_some() && hand2_flush && hand2_straight.is_some() {
        return hand1_straight.unwrap().cmp(&hand2_straight.unwrap());
    }
    if hand1_flush && hand1_straight.is_some() {
        return Ordering::Greater;
    }
    if hand2_flush && hand2_straight.is_some() {
        return Ordering::Less;
    }

    // four of a kind
    let hand1_four = four_of_a_kind(hand1);
    let hand2_four = four_of_a_kind(hand2);
    if hand1_four.is_some() && hand2_four.is_some() {
        let order = hand1_four.unwrap().cmp(&hand2_four.unwrap());
        if order != Ordering::Equal {
            return order;
        }
    }
    if hand1_four.is_some() && hand2_four.is_none() {
        return Ordering::Greater;
    }
    if hand2_four.is_some() && hand1_four.is_none() {
        return Ordering::Less;
    }

    // full house
    let hand1_full = full_house(hand1);
    let hand2_full = full_house(hand2);
    if hand1_full.is_some() && hand2_full.is_some() {
        let (rank1_three, rank1_pair) = hand1_full.unwrap();
        let (rank2_three, rank2_pair) = hand2_full.unwrap();
        let order_triple = rank1_three.cmp(&rank2_three);
        if order_triple != Ordering::Equal {
            return order_triple;
        }
        let order_pair = rank1_pair.cmp(&rank2_pair);
        if order_pair != Ordering::Equal {
            return order_pair;
        }
    }
    if hand1_full.is_some() && hand2_full.is_none() {
        return Ordering::Greater;
    }
    if hand2_full.is_some() && hand1_full.is_none() {
        return Ordering::Less;
    }

    // flush
    if hand1_flush && !hand2_flush {
        return Ordering::Greater;
    }
    if hand2_flush && !hand1_flush {
        return Ordering::Less;
    }

    // straight
    if hand1_straight.is_some() && hand2_straight.is_some() {
        return hand1_straight.unwrap().cmp(&hand2_straight.unwrap());
    }

    // three of a kind
    let hand1_three = three_of_a_kind(hand1);
    let hand2_three = three_of_a_kind(hand2);
    if hand1_three.is_some() && hand2_three.is_some() {
        let order = hand1_three.unwrap().cmp(&hand2_three.unwrap());
        if order != Ordering::Equal {
            return order;
        }
    }
    if hand1_three.is_some() && hand2_three.is_none() {
        return Ordering::Greater;
    }
    if hand2_three.is_some() && hand1_three.is_none() {
        return Ordering::Less;
    }

    // two pair
    let hand1_two_pair = two_pair(hand1);
    let hand2_two_pair = two_pair(hand2);
    if hand1_two_pair.is_some() && hand2_two_pair.is_some() {
        let (rank1_high, rank1_low) = hand1_two_pair.unwrap();
        let (rank2_high, rank2_low) = hand2_two_pair.unwrap();
        let order_high = rank1_high.cmp(&rank2_high);
        if order_high != Ordering::Equal {
            return order_high;
        }
        let order_low = rank1_low.cmp(&rank2_low);
        if order_low != Ordering::Equal {
            return order_low;
        }
    }
    if hand1_two_pair.is_some() && hand2_two_pair.is_none() {
        return Ordering::Greater;
    }
    if hand2_two_pair.is_some() && hand1_two_pair.is_none() {
        return Ordering::Less;
    }

    // pair
    let hand1_pair = pair(hand1);
    let hand2_pair = pair(hand2);
    if hand1_pair.is_some() && hand2_pair.is_some() {
        let order = hand1_pair.unwrap().cmp(&hand2_pair.unwrap());
        if order != Ordering::Equal {
            return order;
        }
    }
    if hand1_pair.is_some() && hand2_pair.is_none() {
        return Ordering::Greater;
    }
    if hand2_pair.is_some() && hand1_pair.is_none() {
        return Ordering::Less;
    }

    // high card
    let hand1_high_rank =
        Rank::try_from(15u8 - extract_ranks(hand1).leading_zeros() as u8).expect("Invalid rank");
    let hand2_high_rank =
        Rank::try_from(15u8 - extract_ranks(hand2).leading_zeros() as u8).expect("Invalid rank");
    hand1_high_rank.cmp(&hand2_high_rank)
}

pub fn straight(hand: u64) -> Option<Rank> {
    let ranks = extract_ranks(hand);
    let straight_mask = 0b11111;

    for i in 0..9 {
        let has_straight = (ranks & (straight_mask << i)).count_ones() == 5;
        if has_straight {
            return Some(Rank::try_from(12 - i).expect("Invalid rank"));
        }
    }

    None
}

pub fn flush(hand: u64) -> bool {
    let suits = extract_suits(hand);
    suits.count_ones() == 1
}

pub fn four_of_a_kind(hand: u64) -> Option<Rank> {
    for rank in (0..13).rev() {
        let rank_mask = RANK_MASK << (rank * 4);
        if (hand & rank_mask).count_ones() == 4 {
            return Some(Rank::try_from(rank).expect("Invalid rank"));
        }
    }

    None
}

pub fn full_house(hand: u64) -> Option<(Rank, Rank)> {
    let three = three_of_a_kind(hand);
    let pair = pair(hand);
    if three.is_none() || pair.is_none() {
        return None;
    }

    return Some((three.unwrap(), pair.unwrap()));
}

pub fn three_of_a_kind(hand: u64) -> Option<Rank> {
    for rank in (0..13).rev() {
        let rank_mask = RANK_MASK << (rank * 4);
        if (hand & rank_mask).count_ones() == 3 {
            return Some(Rank::try_from(rank).expect("Invalid rank"));
        }
    }

    None
}

pub fn two_pair(hand: u64) -> Option<(Rank, Rank)> {
    let high_pair = pair(hand);
    if high_pair.is_none() {
        return None;
    }

    let rest_of_hand = hand & !(RANK_MASK << (high_pair.unwrap() as u8 * 4));
    let low_pair = pair(rest_of_hand);
    if low_pair.is_none() {
        return None;
    }

    Some((high_pair.unwrap(), low_pair.unwrap()))
}

pub fn pair(hand: u64) -> Option<Rank> {
    for rank in (0..13).rev() {
        let rank_mask = RANK_MASK << (rank * 4);
        if (hand & rank_mask).count_ones() == 2 {
            return Some(Rank::try_from(rank).expect("Invalid rank"));
        }
    }

    None
}

fn extract_ranks(hand: u64) -> u16 {
    let mut ranks = 0u16;

    for rank in 0..13 {
        // For each suit in this rank, check if any bit is set
        let rank_mask = RANK_MASK << (rank * 4);
        if hand & rank_mask != 0 {
            ranks |= 1 << rank;
        }
    }

    ranks
}

fn extract_suits(hand: u64) -> u8 {
    let mut suits = 0u8;

    for suit in 0..4 {
        let suit_mask = SUIT_MASK << suit;
        if hand & suit_mask != 0 {
            suits |= 1 << suit;
        }
    }

    suits
}
