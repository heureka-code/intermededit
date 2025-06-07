use std::collections::{HashMap, HashSet};

use intermededit::{AllWords, Word, all_after_one_step};

fn visual_benchmark_without_stopping(by_length: &AllWords, start: Word, max_distance: usize) {
    let mut reached_from = HashMap::<Word, Word>::new();
    let mut current = HashSet::from_iter(vec![&start]);
    let pb = indicatif::ProgressBar::new(max_distance as u64);
    pb.set_style(indicatif::ProgressStyle::with_template("{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(format!("{start} ({max_distance})"));

    for _step_no in 0..max_distance {
        pb.inc(1);
        let mut temp = HashSet::new();
        for rel_start in current {
            for reached in all_after_one_step(by_length, rel_start) {
                reached_from
                    .entry(reached.clone())
                    .or_insert(rel_start.clone());
                temp.insert(reached);
            }
        }

        current = temp;
    }
    pb.finish();
}

pub fn do_timed_way_generation_benchmark(
    all_words: &AllWords,
    tasks: &[(&str, &str)],
    max_distance: usize,
) {
    for (start, _) in tasks {
        visual_benchmark_without_stopping(all_words, Word::new(start), max_distance);
    }
}
