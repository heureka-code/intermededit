use std::{fs::File, io::Write};

use itertools::Itertools;
use rand::prelude::*;

fn main() {
    let wordlist = std::fs::read_to_string("lists/wordlist-orig-no-trailing-spaces.txt").unwrap();
    let mut lines = wordlist.lines().collect_vec();

    let mut out = File::create_new("pairs/test1_most_infty.txt").unwrap();
    lines.shuffle(&mut rand::rng());

    for _ in 1..=10 {
        lines.shuffle(&mut rand::rng());

        for (a, b) in lines.iter().tuples().take(10usize.pow(5)) {
            out.write_fmt(format_args!("{a}\t{b}\n")).unwrap();
        }
    }
}
