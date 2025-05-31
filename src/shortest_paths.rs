use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc;

use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::{MAX_WORD_LEN, Word, WordsOfLength, all_after_one_step};

pub fn find_shortest_paths_from_file(
    filename: &str,
    outfile: BufWriter<File>,
) -> std::io::Result<()> {
    let content = std::fs::read_to_string(filename)?;

    let it = content
        .lines()
        .map(|line| line.split("\t").map(|f| Word::new(f)).collect_vec());
    find_shortest_paths(it, outfile);

    Ok(())
}

pub fn find_shortest_paths(comps: impl Iterator<Item = Vec<Word>>, mut outfile: BufWriter<File>) {
    let (tx, rx) = mpsc::channel();
    let (tx_count, rx_count) = mpsc::channel();
    let writer_thread = std::thread::spawn(move || {
        while let Ok(node_count) = rx_count.recv() {
            let pb = indicatif::ProgressBar::new(node_count as u64);
            pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
            ).unwrap()
        );

            while let Ok(m) = rx.recv() {
                pb.inc(1);
                for ((start, target), distance) in m {
                    outfile
                        .write_fmt(format_args!("{start};{target};{distance}\t"))
                        .unwrap();
                }
                outfile.write_fmt(format_args!("\n")).unwrap();
            }
        }
    });
    for c in comps {
        let mut all_words = vec![WordsOfLength::new(); MAX_WORD_LEN + 2];
        let mut g = UnGraph::<Word, ()>::default();
        let mut nodes: HashMap<&Word, NodeIndex> = HashMap::new();
        for w in c.iter() {
            all_words[w.len()]
                .entry(w.calc_letters())
                .or_default()
                .push(w.clone());
            nodes.insert(w, g.add_node(w.clone()));
        }
        tx_count.send(nodes.len()).unwrap();
        for w in c.iter() {
            let w_node = nodes[w];
            for one in all_after_one_step(&all_words, &w) {
                g.add_edge(w_node, nodes[one], ());
            }
        }

        let shortest_paths = nodes.into_par_iter().map(|(start_word, start_idx)| {
            dijkstra(&g, start_idx, None, |_| 1)
                .into_iter()
                .map(|(target_idx, distance)| {
                    (
                        (
                            start_word.clone(),
                            g.node_weight(target_idx.clone()).unwrap().clone(),
                        ),
                        distance,
                    )
                })
                .collect::<HashMap<_, _>>()
        });

        shortest_paths.for_each(|from_node| {
            tx.send(from_node).unwrap();
        });
    }
    drop(tx);
    drop(tx_count);
    writer_thread.join().unwrap();
}
