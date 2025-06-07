use std::thread::JoinHandle;

use intermededit::{
    AllWords, MAX_WORD_LEN, Word,
    base::{LetterVariationsPerOperation, QueryableWordbucketList},
    expect_wordlist,
    operations::{Delete, Insert, Operation, Replace},
    step_generation::{FilterWordsForOperation, find_after_operation},
};

use std::fs::File;
use std::io::{BufWriter, Write};

fn edges_for_operation<Op: Operation + FilterWordsForOperation + LetterVariationsPerOperation>(
    all_words: &AllWords,
    len: usize,
) -> std::io::Result<()> {
    let target_len = (len as i32) + (<Op as Operation>::len_delta());
    let pb = indicatif::ProgressBar::no_length();
    let file = File::create_new(format!(
        "{}-{len:02}-{target_len:02}.txt",
        <Op as Operation>::lowercase_label()
    ))?;
    let mut buffered = BufWriter::new(file);

    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos} ({per_sec}, {eta})",
        )
        .unwrap(),
    );
    pb.set_message(format!("{len:02}->{target_len:02}: "));

    for (s, t) in all_words.iter_all().flat_map(|start: &Word| {
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

fn generate_thread<Op: FilterWordsForOperation + LetterVariationsPerOperation>(
    all_words: &AllWords,
) -> JoinHandle<()> {
    let all_words = all_words.clone();
    std::thread::spawn(move || {
        for len in 0..MAX_WORD_LEN {
            edges_for_operation::<Op>(&all_words, len).unwrap();
        }
    })
}

pub fn concurrent_edge_file_creation(all_words: &AllWords) {
    let i = generate_thread::<Insert>(all_words);
    let d = generate_thread::<Delete>(all_words);
    let r = generate_thread::<Replace>(all_words);

    i.join().unwrap();
    d.join().unwrap();
    r.join().unwrap();
}
