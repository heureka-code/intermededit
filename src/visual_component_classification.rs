use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{collections::HashSet, fs::File, io::BufWriter};

use intermededit::components::*;
use intermededit::{AllWords, Word, get_word_count};
use itertools::Itertools;

fn classify_words_file_writer_thread(
    kind: ComponentAnalysis,
    file: BufWriter<File>,
    rx_data: Receiver<HashSet<Word>>,
    tx_progress: Sender<(ComponentAnalysis, usize)>,
) -> JoinHandle<std::io::Result<()>> {
    std::thread::spawn(move || {
        let mut writer = file;
        while let Ok(d) = rx_data.recv() {
            writer.write_fmt(format_args!("{}\n", d.iter().join("\t")))?;
            let _ = tx_progress.send((kind, d.len()));
        }
        Ok(())
    })
}

#[allow(unused)]
pub fn visual_classify_words_exhaustive(all_words: AllWords, single_components: BufWriter<File>) {
    visual_classify_words_helper(all_words, usize::MAX, single_components, None);
}

#[allow(unused)]
pub fn visual_classify_words(
    all_words: AllWords,
    max_distance: usize,
    single_components: BufWriter<File>,
    too_big_components: BufWriter<File>,
) {
    visual_classify_words_helper(
        all_words,
        max_distance,
        single_components,
        Some(too_big_components),
    );
}

fn visual_classify_words_helper(
    all_words: AllWords,
    max_distance: usize,
    single_components: BufWriter<File>,
    too_big_components: Option<BufWriter<File>>,
) {
    let total_word_count = get_word_count(&all_words);

    let (single_complete_thread, big_unknown_thread, pb_thread) = {
        let (tx_progress, rx_progress) = std::sync::mpsc::channel::<(ComponentAnalysis, usize)>();
        let (tx_single, rx_single) = std::sync::mpsc::channel::<HashSet<Word>>();
        let (tx_unknown, rx_unknown) = std::sync::mpsc::channel::<HashSet<Word>>();

        let pb_thread = classify_word_progress_bar_thread(total_word_count as u64, rx_progress);
        let single_complete_thread = classify_words_file_writer_thread(
            ComponentAnalysis::DefinitlyComplete,
            single_components,
            rx_single,
            tx_progress.clone(),
        );
        let big_unknown_thread = too_big_components.map(|too_big_components: BufWriter<File>| {
            classify_words_file_writer_thread(
                ComponentAnalysis::TooBig,
                too_big_components,
                rx_unknown,
                tx_progress,
            )
        });

        classify_words_into_components(all_words, max_distance, tx_single, tx_unknown);

        (single_complete_thread, big_unknown_thread, pb_thread)
    };
    single_complete_thread.join().unwrap().unwrap();
    if let Some(t) = big_unknown_thread {
        t.join().unwrap().unwrap()
    }
    pb_thread.join().unwrap();
}

fn classify_word_progress_bar_thread(
    total_word_count: u64,
    rx_progress: Receiver<(ComponentAnalysis, usize)>,
) -> JoinHandle<()> {
    let pb = indicatif::ProgressBar::new(total_word_count);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
        ).unwrap()
        //.progress_chars("#>-")
    );

    std::thread::spawn(move || {
        let mut complete = 0;
        let mut too_big = 0;
        let mut c_buckets = 0;
        let mut b_buckets = 0;
        while let Ok((kind, count)) = rx_progress.recv() {
            match kind {
                ComponentAnalysis::TooBig => {
                    too_big += count;
                    b_buckets += 1;
                }
                ComponentAnalysis::DefinitlyComplete => {
                    complete += count;
                    c_buckets += 1;
                }
            }
            pb.inc(count as u64);
            pb.set_message(format!(
                "complete: {complete} in {c_buckets}, unknown: {too_big} in {b_buckets}"
            ));
        }
        pb.finish();
    })
}
