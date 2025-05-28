use std::fs::File;
use std::io::{BufWriter, Write};

use crate::one_step::{find_after_deletion, find_after_insertion, find_after_substitution};
use crate::{AllWords, Word};

enum Mode {
    Insert = 1,
    Replace = 0,
    Delete = -1,
}

macro_rules! find_variant {
    ($name: ident, $label: expr, $function: ident, $mode: expr) => {
    pub fn $name(all_words: &AllWords, len: usize) -> std::io::Result<()> {
        let target_len = (len as i32) + ($mode as i32);
        let bucket = &all_words[len];
        let pb = indicatif::ProgressBar::no_length();
        let file = File::create_new(format!("{}-{len:02}-{target_len:02}.txt", $label))?;
        let mut buffered = BufWriter::new(file);

        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos} ({per_sec}, {eta})"
            )
            .unwrap()
        );
        pb.set_message(format!("{len:02}->{target_len:02}: "));

        for (s, t) in bucket.values().flatten().flat_map(|start: &Word| {
            $function(all_words, start).map(move |target| (start, target))
        }) {
            if s != t {
                pb.inc(1);
                buffered.write_fmt(format_args!("{s}\t{t}\n"))?;
            }
        }
        pb.finish();
        Ok(())
        }
    };
}

find_variant!(
    edges_for_substitution,
    "replace",
    find_after_substitution,
    Mode::Replace
);
find_variant!(
    edges_for_insertion,
    "insert",
    find_after_insertion,
    Mode::Insert
);
find_variant!(
    edges_for_deletion,
    "delete",
    find_after_deletion,
    Mode::Delete
);
