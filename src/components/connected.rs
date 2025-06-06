use std::collections::HashMap;

use itertools::Itertools;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

pub struct ConnectedComponents {
    #[allow(unused)]
    uf: QuickUnionUf<UnionBySize>,
    pub words: HashMap<u32, Vec<u32>>,
    pub edges: HashMap<u32, Vec<(u32, u32)>>,
    empty: Vec<(u32, u32)>,
}
impl ConnectedComponents {
    pub fn new(word_count: usize, all_edges: &[(u32, u32)]) -> Self {
        let mut uf = QuickUnionUf::<UnionBySize>::new(word_count);
        for (s, t) in all_edges {
            uf.union(*s as usize, *t as usize);
        }
        let components_words = (0..word_count)
            .map(|i| (uf.find(i) as u32, i as u32))
            .into_group_map();
        let components_edges = all_edges
            .iter()
            .map(|(s, t)| (uf.find(*s as usize) as u32, (*s, *t)))
            .into_group_map();
        Self {
            uf,
            words: components_words,
            edges: components_edges,
            empty: Vec::new(),
        }
    }
    pub fn words_edges(&self) -> impl Iterator<Item = (&Vec<u32>, &Vec<(u32, u32)>)> {
        self.words
            .iter()
            .map(|(key, w)| (w, self.edges.get(key).unwrap_or(&self.empty)))
    }
    pub fn take_words_edges(mut self) -> impl Iterator<Item = (Vec<u32>, Vec<(u32, u32)>)> {
        self.words.into_iter().map(move |(key, w)| {
            if let Some(e) = self.edges.get_mut(&key) {
                (w, std::mem::take(e))
            } else {
                (w, vec![])
            }
        })
    }
}
