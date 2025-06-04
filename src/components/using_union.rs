use union_find::{QuickUnionUf, UnionBySize, UnionFind};

use crate::wordbucket_impls::NumberedLenLetWordlist;
use crate::{all_after_one_step, base::QueryableWordbucketList};

/// assumes each word is numbered with a unique number starting by 0 without gaps till COUNT-1
pub fn union_find_classify_words_into_components(
    all_words: &NumberedLenLetWordlist,
) -> QuickUnionUf<UnionBySize> {
    let mut uf = QuickUnionUf::<UnionBySize>::new(all_words.get_word_count());

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let word_count = all_words.get_word_count();
    let _thread = std::thread::spawn(move || {
        let pb = indicatif::ProgressBar::new(word_count as u64);
        pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
            ).unwrap()
        );
        while let Ok(_) = rx.recv() {
            pb.inc(1);
        }
        pb.finish();
    });

    for start in all_words.iter_all() {
        // let _ = tx.send(());
        for target in all_after_one_step(all_words, start) {
            uf.union(start.ctag(), target.ctag());
        }
    }
    uf
}
