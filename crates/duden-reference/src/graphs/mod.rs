mod from_edge_iterator;
mod maybe_cached;
mod s_components;
pub use s_components::{Component, Components, SComp, SComponents, WComp, WComponents};

use from_edge_iterator::nodes_edges_into_compressed_graph;
pub use maybe_cached::get_maybe_cached_graph;

use std::hash::Hash;

use petgraph::{Directed, Graph};

use crate::{FindAfterOperation, HasWord, QueryableWordbucketList};

pub trait WordlistToGraph {
    fn wordlist_to_default_edge_graph<'a, L>(
        wordlist: &'a L,
    ) -> Graph<<L as QueryableWordbucketList<'a>>::W, (), Directed, u32>
    where
        L: QueryableWordbucketList<'a>,
        <L as QueryableWordbucketList<'a>>::W: Eq + Hash + Clone,
    {
        Self::wordlist_to_edge_kind_graph(wordlist)
    }
    fn wordlist_to_undirected_graph<'a, L>(
        wordlist: &'a L,
    ) -> Graph<<L as QueryableWordbucketList<'a>>::W, (), petgraph::Undirected, u32>
    where
        L: QueryableWordbucketList<'a>,
        <L as QueryableWordbucketList<'a>>::W: Eq + Hash + Clone,
    {
        Self::wordlist_to_edge_kind_graph(wordlist)
    }
    fn wordlist_to_directed_graph<'a, L>(
        wordlist: &'a L,
    ) -> Graph<<L as QueryableWordbucketList<'a>>::W, (), petgraph::Directed, u32>
    where
        L: QueryableWordbucketList<'a>,
        <L as QueryableWordbucketList<'a>>::W: Eq + Hash + Clone,
    {
        Self::wordlist_to_edge_kind_graph(wordlist)
    }
    fn wordlist_to_edge_kind_graph<'a, L, EdgeKind>(
        wordlist: &'a L,
    ) -> Graph<<L as QueryableWordbucketList<'a>>::W, (), EdgeKind, u32>
    where
        EdgeKind: petgraph::EdgeType,
        L: QueryableWordbucketList<'a>,
        <L as QueryableWordbucketList<'a>>::W: Eq + Hash + Clone;
}

impl<O: FindAfterOperation> WordlistToGraph for O {
    fn wordlist_to_edge_kind_graph<'a, L, EdgeKind>(
        wordlist: &'a L,
    ) -> Graph<<L as QueryableWordbucketList<'a>>::W, (), EdgeKind, u32>
    where
        EdgeKind: petgraph::EdgeType,
        L: QueryableWordbucketList<'a>,
        <L as QueryableWordbucketList<'a>>::W: Eq + Hash + Clone,
    {
        let edges = wordlist.iter_all().flat_map(|word| {
            O::find_after_operation(wordlist, word.word()).map(|step| (word.clone(), step.clone()))
        });
        nodes_edges_into_compressed_graph(
            wordlist.iter_all().cloned(),
            edges,
            wordlist.get_word_count(),
        )
    }
}
