use std::fs::File;
use std::io::{BufWriter, Write};

use crate::base::one_step::find_after_operation;
use crate::base::operations;
use crate::{AllWords, Word};

macro_rules! find_variant {
    ($name: ident, $mode: ty) => {
    pub fn $name(all_words: &AllWords, len: usize) -> std::io::Result<()> {
        let target_len = (len as i32) + (<$mode as $crate::base::operations::Operation>::len_delta());
        let bucket = &all_words[len];
        let pb = indicatif::ProgressBar::no_length();
        let file = File::create_new(format!("{}-{len:02}-{target_len:02}.txt", <$mode as $crate::base::operations::Operation>::lowercase_label()))?;
        let mut buffered = BufWriter::new(file);

        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos} ({per_sec}, {eta})"
            )
            .unwrap()
        );
        pb.set_message(format!("{len:02}->{target_len:02}: "));

        for (s, t) in bucket.values().flatten().flat_map(|start: &Word| {
            find_after_operation::<$mode>(all_words, start).map(move |target| (start, target))
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

find_variant!(edges_for_substitution, operations::Replace);
find_variant!(edges_for_insertion, operations::Insert);
find_variant!(edges_for_deletion, operations::Delete);
