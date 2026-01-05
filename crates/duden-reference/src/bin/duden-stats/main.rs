mod comp_correlation_heatmaps;
mod component_info;
mod compute_per_graph_variant;
mod diameter_center;
mod general_wordlist;
mod msg_ctx;
use msg_ctx::MSender;

use std::collections::HashMap;
use std::sync::Arc;

use compute_per_graph_variant::compute;
use general_wordlist::*;

use duden_reference::prelude::*;
use duden_reference::{Word, operations as op};

fn main() {
    let file_content = std::fs::read_to_string("lists/wordlist-orig-no-trailing-spaces.txt")
        .unwrap()
        .to_lowercase();

    let mut wordlist = duden_reference::TaggedLenLetWordlist::new();
    let mut word_lookup = HashMap::new();

    for line in file_content.lines() {
        if word_lookup.contains_key(line) {
            println!("Word already exists: {line}");
            continue;
        }
        let word = Word::new(line).unwrap().number_with(());
        let word = wordlist.insert_new_with_ref(word);
        word_lookup.insert(line, *word);
    }
    assert_eq!(word_lookup.len(), wordlist.iter_all().count());
    println!("wordlist complete");

    let (tx, rx) = crate::MSender::channel();

    let _ = wordlist_distribution_analysis(tx.scope("wordlist"), &wordlist);

    std::thread::spawn(move || {
        while let Ok((category, msg)) = rx.recv() {
            println!("[{category}] {msg}");
        }
    });

    let rayon_access = Arc::new(std::sync::Mutex::new(()));

    macro_rules! spawn_it {
        ($s:ident : $ty: ty) => {{
            let rayon_access = rayon_access.clone();
            $s.spawn(|| compute::<$ty>(tx.clone(), rayon_access, &wordlist, &word_lookup));
        }};
    }

    std::thread::scope(|s| {
        spawn_it!(s : op::InsertReplaceDelete);
        spawn_it!(s : op::InsertReplace);
        spawn_it!(s : op::InsertDelete);
        spawn_it!(s : op::Insert);
        spawn_it!(s : op::ReplaceDelete);
        spawn_it!(s : op::Replace);
        spawn_it!(s : op::Delete);
    });
    tx.send("successfully completed all tasks, terminating...");
}
