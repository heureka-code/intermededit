use std::io::Write;

use duden_reference::{QueryableWordbucketList, TaggedLenLetWordlist};
use itertools::Itertools;

use crate::msg_ctx::MSender;

pub fn wordlist_distribution_analysis<N>(
    messages: MSender,
    wordlist: &TaggedLenLetWordlist<'_, N>,
) -> std::io::Result<()> {
    messages.send("start");
    let _ = std::fs::create_dir_all("data/word-cat-dist");
    let mut out = std::fs::File::create("data/word-cat-dist/heatmap.data").unwrap();
    let mut category_count = 0;
    for i in wordlist.iter_lengths() {
        out.write_fmt(format_args!(
            "{}\n",
            i.iter()
                .map(|(letters, v)| (letters, v.len()))
                .sorted()
                .map(|(_, l)| l)
                .join(",")
        ))?;
        category_count += i.len();
    }
    let avg = wordlist.get_word_count() as f64 / category_count as f64;
    messages.send(format!("{avg:.3} words per category in average"));
    Ok(())
}
