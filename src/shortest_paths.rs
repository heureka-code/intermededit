use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc;

use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::prelude::*;

use crate::base::InsertWordbucketList;
use crate::wordbucket_impls::LenLetWordlist;
use crate::{Word, all_after_one_step};

pub fn find_shortest_paths_from_file(
    filename: &str,
    outfile: BufWriter<File>,
) -> std::io::Result<()> {
    let content = std::fs::read_to_string(filename)?;

    let it = content
        .lines()
        .map(|line| line.split("\t").map(Word::new).collect_vec());
    find_shortest_paths(it, outfile);

    Ok(())
}

pub fn find_shortest_paths(comps: impl Iterator<Item = Vec<Word>>, mut outfile: BufWriter<File>) {
    let (tx, rx) = mpsc::channel();
    let writer_thread = std::thread::spawn(move || {
        while let Ok(either::Either::Left((idx, node_count))) = rx.recv() {
            let pb = indicatif::ProgressBar::new(node_count as u64);
            pb.set_style(
            indicatif::ProgressStyle::with_template(
                "{msg} [{elapsed_precise:.green}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}, {eta})"
            ).unwrap()
        );
            if node_count > 200 {
                pb.set_message(format!("Component {idx}:"));
            }

            let mut o = false;
            while let Ok(either::Either::Right(((start, target), distance))) = rx.recv() {
                if node_count > 200 {
                    pb.inc(1);
                }
                outfile
                    .write_fmt(format_args!("{start};{target};{distance}\t"))
                    .unwrap();
                o = true;
            }
            if o {
                if node_count > 200 {
                    pb.finish();
                }
                outfile.write_fmt(format_args!("\n")).unwrap();
            }
        }
    });
    for (comp_idx, c) in comps.enumerate() {
        let mut all_words = LenLetWordlist::default();
        let mut g = UnGraph::<Word, ()>::default();
        let mut nodes: HashMap<&Word, NodeIndex> = HashMap::new();
        for w in c.iter() {
            all_words.insert_new(w.clone());
            nodes.insert(w, g.add_node(w.clone()));
        }
        tx.send(either::Either::Left((comp_idx, nodes.len())))
            .unwrap();
        for w in c.iter() {
            let w_node = nodes[w];
            for one in all_after_one_step(&all_words, w) {
                g.add_edge(w_node, nodes[one], ());
            }
        }

        let shortest_paths = nodes
            .into_par_iter()
            .map(|(start_word, start_idx)| (start_word, dijkstra(&g, start_idx, None, |_| 1)));

        shortest_paths.for_each(|(start, reached)| {
            if reached.len() < 4 {
                return;
            }
            let (target_idx, distance) = reached
                .iter()
                .max_by(|(_a, da), (_b, db)| da.cmp(db))
                .unwrap();
            let target = g.node_weight(*target_idx).unwrap().clone();
            tx.send(either::Either::Right(((start.clone(), target), *distance)))
                .unwrap();
        });
        tx.send(either::Either::Left((0, 0))).unwrap();
    }
    drop(tx);
    writer_thread.join().unwrap();
}
