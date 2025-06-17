use std::fs::File;
use std::io::{BufWriter, Write};

use plotto_utils::utils::all_hands::all_hands;

fn main() {
    let hands = all_hands();

    let out_path = std::path::Path::new("src/generated_hands.rs");
    let file = File::create(out_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "pub static SORTED_HANDS: &[u64] = &[").unwrap();

    for hand in hands {
        writeln!(writer, "    {hand},").unwrap();
    }

    writeln!(writer, "];").unwrap();
    println!("cargo:rerun-if-changed=../utils/src/utils/all_hands.rs");
    println!("cargo:rerun-if-changed=../utils/src/utils/compare_hands.rs");
}
