use crate::base::WordlistExt;
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{collections::HashSet, fs::File, io::BufWriter};

use crate::{AllWords, Word};
use itertools::Itertools;

use super::BfsWordComponentClassification;

fn classify_words_file_writer_thread(
    file: BufWriter<File>,
    rx_data: Receiver<HashSet<Word>>,
    tx_progress: Sender<usize>,
) -> JoinHandle<std::io::Result<()>> {
    std::thread::spawn(move || {
        let mut writer = file;
        while let Ok(d) = rx_data.recv() {
            #[cfg(feature = "words-in-components-sorted")]
            let line = d.iter().sorted().join("\t");
            #[cfg(not(feature = "words-in-components-sorted"))]
            let line = d.iter().join("\t");

            writer.write_fmt(format_args!("{line}\n"))?;
            let _ = tx_progress.send(d.len());
        }
        Ok(())
    })
}

#[allow(unused)]
pub fn bfs_visual_classify_words_exhaustive(
    all_words: AllWords,
    single_components: BufWriter<File>,
) {
    let total_word_count = all_words.get_word_count();

    let (single_complete_thread, pb_thread) = {
        let (tx_progress, rx_progress) = std::sync::mpsc::channel::<usize>();
        let (tx_single, rx_single) = std::sync::mpsc::channel::<HashSet<Word>>();

        let pb_thread = classify_word_progress_bar_thread(total_word_count as u64, rx_progress);
        let single_complete_thread = classify_words_file_writer_thread(
            single_components,
            rx_single,
            tx_progress.clone(),
        );
        BfsWordComponentClassification::new_max()
            .classify_words_into_components(all_words, tx_single)
            .expect("No too big components");

        (single_complete_thread, pb_thread)
    };
    single_complete_thread.join().unwrap().unwrap();
    pb_thread.join().unwrap();
}

fn classify_word_progress_bar_thread(
    total_word_count: u64,
    rx_progress: Receiver<usize>,
) -> JoinHandle<()> {
    let pb = indicatif::ProgressBar::new(total_word_count);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
        ).unwrap()
    );

    std::thread::spawn(move || {
        let mut complete = 0;
        let mut buckets = 0;
        while let Ok(count) = rx_progress.recv() {
            complete += count;
            buckets += 1;
            pb.inc(count as u64);
            pb.set_message(format!(
                "complete: {complete} in {buckets}"
            ));
        }
        pb.finish();
    })
}
