use itertools::Itertools;
use rayon::prelude::*;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::wordbucket_impls::NumberedLenLetWordlist;
use crate::{all_after_one_step, base::QueryableWordbucketList};

/// assumes each word is numbered with a unique number starting by 0 without gaps till COUNT-1
pub fn union_find_classify_words_into_components(
    all_words: &NumberedLenLetWordlist,
) -> QuickUnionUf<UnionBySize> {
    let uf = QuickUnionUf::<UnionBySize>::new(all_words.get_word_count());

    let (tx_pb, rx_pb) = std::sync::mpsc::channel();
    let (tx, rx) = std::sync::mpsc::channel();

    let word_count = all_words.get_word_count();
    let _thread = std::thread::spawn(move || {
        let pb = indicatif::ProgressBar::new(word_count as u64);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
            ).unwrap()
        );
        while let Ok(msg) = rx_pb.recv() {
            if let either::Either::Left(inc) = msg {
                pb.inc(inc);
            } else if let Some(len) = pb.length() {
                pb.set_position(len);
            }
        }
        pb.finish();
    });

    let union_thread = std::thread::spawn({
        let tx_pb = tx_pb.clone();
        move || {
        let mut uf = uf;
        while let Ok((start, targets)) = rx.recv() {
            for target in targets {
                uf.union(start, target);
            }
            let _ = tx_pb.send(either::Either::Left(1));
        }
        uf
    }});

    all_words.iter_all().par_bridge().for_each(|start| {
        let _ = tx.send((start.ctag(), all_after_one_step(all_words, start).map(|w| w.ctag()).collect_vec()));
    });
    let _ = tx_pb.send(either::Either::Right(()));
    drop(tx);
    drop(tx_pb);
    let _ = _thread.join();
    union_thread.join().unwrap()
}
