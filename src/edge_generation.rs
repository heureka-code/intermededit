use std::fs::File;
use std::io::{BufWriter, Write};

use crate::base::model::letters::LetterVariationsPerOperation;
use crate::base::one_step::{FilterWordsForOperation, find_after_operation};
use crate::operations::Operation;
use crate::{AllWords, Word};

pub fn edges_for_operation<
    Op: Operation + FilterWordsForOperation + LetterVariationsPerOperation,
>(
    all_words: &AllWords,
    len: usize,
) -> std::io::Result<()> {
    let target_len = (len as i32) + (<Op as crate::base::operations::Operation>::len_delta());
    let bucket = &all_words[len];
    let pb = indicatif::ProgressBar::no_length();
    let file = File::create_new(format!(
        "{}-{len:02}-{target_len:02}.txt",
        <Op as crate::base::operations::Operation>::lowercase_label()
    ))?;
    let mut buffered = BufWriter::new(file);

    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos} ({per_sec}, {eta})",
        )
        .unwrap(),
    );
    pb.set_message(format!("{len:02}->{target_len:02}: "));

    for (s, t) in bucket.values().flatten().flat_map(|start: &Word| {
        find_after_operation::<Op, AllWords>(all_words, start).map(move |target| (start, target))
    }) {
        if s != t {
            pb.inc(1);
            buffered.write_fmt(format_args!("{s}\t{t}\n"))?;
        }
    }
    pb.finish();
    Ok(())
}
