use std::{collections::HashMap, io::Write};

use petgraph::graph::DiGraph;

use crate::{HasWord, TWord, graphs::WordlistToGraph};

pub fn get_maybe_cached_graph<
    'a,
    O: WordlistToGraph,
    N: std::cmp::Eq + std::hash::Hash + std::clone::Clone + Copy,
>(
    file: &str,
    wordlist: &'a crate::TaggedLenLetWordlist<N>,
    word_lookup: &HashMap<&str, TWord<'a, N>>,
) -> DiGraph<TWord<'a, N>, (), u32> {
    if std::fs::exists(file).unwrap() {
        let data = std::fs::read_to_string(file).unwrap();
        let node_count: usize = data.lines().next().unwrap().parse().unwrap();

        let graph: DiGraph<TWord<'a, N>, (), u32> =
            crate::graphs::nodes_edges_into_compressed_graph(
                data.lines()
                    .skip(1)
                    .take(node_count)
                    .map(|line| *word_lookup.get(line).unwrap()),
                data.lines().skip(node_count + 1).map(|line| {
                    let (a, b) = line.split_once(";").unwrap();
                    (*word_lookup.get(a).unwrap(), *word_lookup.get(b).unwrap())
                }),
                word_lookup.len(),
            );
        graph
    } else {
        let mut out = std::fs::File::create_new(file).unwrap();
        let graph = O::wordlist_to_directed_graph(wordlist);

        out.write_fmt(format_args!("{}\n", graph.node_count()))
            .unwrap();
        for n in graph.node_weights() {
            out.write_fmt(format_args!("{}\n", n.word())).unwrap();
        }

        for c in graph.edge_indices() {
            let (ni1, ni2) = graph.edge_endpoints(c).unwrap();
            let (n1, n2) = (
                graph.node_weight(ni1).unwrap(),
                graph.node_weight(ni2).unwrap(),
            );
            out.write_fmt(format_args!("{};{}\n", n1.word(), n2.word()))
                .unwrap();
        }
        graph
    }
}
