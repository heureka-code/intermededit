use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    hint::black_box,
};

use intermededit::{
    Letters, Word,
    base::{
        FindAfterOperation, HasWord, QueryableWordbucketList, TWord,
        operations::InsertReplaceDelete,
    },
};

fn main() {
    let args = std::env::args().collect_vec();

    let wordlist_file = std::fs::read_to_string(args[1].clone())
        .unwrap()
        .to_lowercase();

    let mut wordlist = intermededit::TaggedLenLetWordlist::new();
    let mut word_lookup = HashMap::new();

    for line in wordlist_file.lines() {
        if word_lookup.contains_key(line) {
            continue;
        }
        let word = Word::new(line).unwrap().number_with(());
        let word = wordlist.insert_new_with_ref(word);
        word_lookup.insert(line, *word);
    }
    assert_eq!(word_lookup.len(), wordlist.iter_all().count());
    eprintln!("wordlist complete");
    eprintln!("{args:?}");

    let task_file = std::fs::read_to_string(args[2].clone()).unwrap();

    let task_to_solution_line = |task| {
        let way = way_for(&wordlist, &word_lookup, task);
        if let Some(mut way) = way {
            let mut w = way.join(";");
            w.push('\n');
            w
        } else {
            ("-\n").to_string()
        }
    };
    let concat_strings = |mut a: String, mut b: String| {
        a.extend(b.drain(..));
        a
    };

    let solutions = task_file
        .par_lines()
        .map(task_to_solution_line)
        .reduce(String::new, concat_strings);
    print!("{solutions}");
}

fn way_for<'a, N: Eq + Hash + std::fmt::Debug>(
    list: &'a intermededit::TaggedLenLetWordlist<'a, N>,
    lookup: &'a HashMap<&'a str, TWord<'a, N>>,
    task: &str,
) -> Option<impl Iterator<Item = &'a Word<'a>>> {
    let (from, to) = task.split(";").take(2).collect_tuple().unwrap();
    Some(
        find_way(list, &lookup[from], &lookup[to])?
            .into_iter()
            .map(|s| s.word()),
    )
}

pub fn find_way<'a, N: Eq + Hash + std::fmt::Debug>(
    list: &'a intermededit::TaggedLenLetWordlist<'a, N>,
    from: &'a TWord<'_, N>,
    target: &'a TWord<'_, N>,
) -> Option<Vec<&'a Word<'a>>> {
    let mut reached_from = HashMap::new();
    let mut current: HashSet<&'a TWord<'_, N>> = HashSet::from_iter(vec![from]);
    let mut seen = HashSet::new();

    loop {
        let mut temp = HashSet::new();
        for rel_start in current {
            if !seen.insert(rel_start) {
                continue;
            }
            for reached in InsertReplaceDelete::find_after_operation(list, rel_start.word()) {
                reached_from
                    .entry(reached.word())
                    .or_insert(rel_start.word());
                temp.insert(reached);
                if reached.word() == target.word() {
                    let mut way = vec![target.word()];
                    while let Some(last) = way.last()
                        && last != &from.word()
                    {
                        way.push(reached_from.get(&last.word()).unwrap());
                    }

                    way.reverse();
                    return Some(way);
                }
            }
        }
        current = temp;
    }
}

fn _test() {
    let (content, pairs) = {
        let content = std::thread::spawn(|| {
            std::fs::read_to_string("lists/wordlist-orig-no-trailing-spaces.txt")
                .unwrap()
                .to_lowercase()
        });

        let pairs = std::thread::spawn(|| {
            std::fs::read_to_string("pairs/test1_most_infty.txt")
                .unwrap()
                .to_lowercase()
        });
        (content.join(), pairs.join())
    };
    let content = content.unwrap();
    let pairs = pairs.unwrap();

    let letters = content
        .par_lines()
        .map(|c| unsafe {
            let word = Word::new_unchecked(c);
            (c.len(), word.calc_letters(), word)
        })
        .collect_vec_list();

    // let mut mapping: Vec<HashMap<Letters, Vec<Word<'_>>>> = vec![HashMap::new(); 100];

    let xxx =
        move |length, letters: &std::collections::LinkedList<Vec<(usize, Letters, Word<'_>)>>| {
            let mut hm: HashMap<Letters, Vec<Word<'_>>> = HashMap::new();

            for v in letters {
                for (len, lett, word) in v {
                    if *len == length {
                        hm.entry(*lett).or_default().push(*word);
                    }
                }
            }
        };
    let mapping: HashMap<usize, _> = (1..100)
        .par_bridge()
        .map(|length| (length, xxx(length, &letters)))
        .collect();

    /*
    for coll in letters.into_iter() {
        for (len, lett, word) in coll.into_iter() {
            mapping[len].entry(lett).or_default().push(word);
        }
    }*/

    let _exercises: Vec<(Word<'_>, Word<'_>)> = pairs
        .par_lines()
        .map(|c| unsafe {
            let (from, to) = c.split_once('\t').unwrap_unchecked();
            (Word::new_unchecked(from), Word::new_unchecked(to))
        })
        .collect();

    let mapping = black_box(mapping);
    println!("{:?}", mapping.len());

    // let _v = black_box((&letters, &pairs));
    // println!("{}", letters.len());
    // println!("{:?}", letters.iter().next());
    // println!("{exercises:?}");
    // println!("{:?}", letters.into_iter().next());
}
