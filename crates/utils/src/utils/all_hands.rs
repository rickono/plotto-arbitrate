use crate::utils::compare_hands::compare_hands;

pub fn all_hands() -> Vec<u64> {
    let mut combos = Vec::with_capacity(2_598_960);
    let mut bits: u64 = 0b11111;

    while bits < (1 << 52) {
        combos.push(bits);

        let c = bits & bits.wrapping_neg();
        let r = bits + c;
        bits = (((r ^ bits) >> 2) / c) | r;
    }

    combos.sort_by(|a, b| compare_hands(*b, *a));

    combos
}
