use std::{collections::HashMap, hash::Hash, path::Path, time::Instant};

use duden_reference::{
    FindAfterOperation, HasWord, QueryableWordbucketList, TWord, Word,
    operations::InsertReplaceDelete,
};
use itertools::Itertools;

// mod validation;

#[derive(Debug)]
pub enum WayValidationErr<'a> {
    ShorterThanPossible,
    NonExistantWord(&'a str),
    InvalidStep(usize),
    TooLong { shortest: usize },
}

pub fn validate_steps<'a, O: FindAfterOperation, N>(
    list: &'a duden_reference::TaggedLenLetWordlist<'a, N>,
    lookup: &'a HashMap<&'a str, TWord<'a, N>>,
    way: &[&'a str],
) -> Result<(), WayValidationErr<'a>>
where
    N: PartialEq + Eq + Hash,
{
    if way.len() <= 1 {
        return Ok(());
    }
    let mut steps = Vec::with_capacity(way.len());
    for word in way.iter() {
        if let Some(nw) = lookup.get(word) {
            steps.push(nw);
        } else {
            return Err(WayValidationErr::NonExistantWord(word));
        }
    }
    for (idx, (from, to)) in steps.iter().tuple_windows().enumerate() {
        if from.word().len().abs_diff(to.word().len()) > 1 {
            return Err(WayValidationErr::InvalidStep(idx));
        }
        let valid = O::find_after_operation(list, from.word()).contains(to);
        if !valid {
            return Err(WayValidationErr::InvalidStep(idx));
        }
    }
    Ok(())
}

fn main() {
    let current_dir = std::env::current_dir().unwrap().canonicalize().unwrap();

    let mut wordlist_path = current_dir.clone();
    wordlist_path.push("lists");
    wordlist_path.push("wordlist-orig-no-trailing-spaces.txt");

    let mut tasks_path = current_dir.clone();
    tasks_path.push("sheet-tasks.txt");

    let file_content = std::fs::read_to_string(&wordlist_path)
        .unwrap()
        .to_lowercase();

    let mut wordlist = duden_reference::TaggedLenLetWordlist::new();
    let mut word_lookup = HashMap::new();

    for line in file_content.lines() {
        if word_lookup.contains_key(line) {
            // println!("Word already exists: {line}");
            continue;
        }
        let word = Word::new(line).unwrap().number_with(());
        let word = wordlist.insert_new_with_ref(word);
        word_lookup.insert(line, *word);
    }
    assert_eq!(word_lookup.len(), wordlist.iter_all().count());

    let out = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(["--bin", "intermededit"])
        .current_dir("../..")
        .output()
        .unwrap();
    if !out.status.success() {
        println!("\n{}\n", String::from_utf8_lossy(&out.stderr));
    }

    validate_tasks(
        &wordlist,
        &word_lookup,
        wordlist_path,
        tasks_path,
        "target/release/intermededit",
        "../..",
    );
}
fn validate_tasks<N: Eq + Hash>(
    list: &duden_reference::TaggedLenLetWordlist<'_, N>,
    lookup: &HashMap<&str, TWord<'_, N>>,
    wordlist_path: impl AsRef<Path>,
    tasks_path: impl AsRef<Path>,
    binary: impl AsRef<Path>,
    current_dir: impl AsRef<Path>,
) {
    let wordlist_path = wordlist_path.as_ref();
    let tasks_path = tasks_path.as_ref();
    let binary = binary.as_ref();
    let tasks = std::fs::read_to_string(tasks_path).unwrap();

    let time = Instant::now();
    let process = std::process::Command::new(binary)
        .arg(wordlist_path)
        .arg(tasks_path)
        .current_dir(current_dir.as_ref())
        .output()
        .expect("error in running binary");
    println!("Elapsed: {:?}", time.elapsed());

    if !process.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&process.stderr));
        println!("Abnormal termination {}", process.status);
        eprintln!("{}", String::from_utf8_lossy(&process.stdout));
        std::process::exit(4);
    }

    let Ok(program_out) = String::from_utf8(process.stdout) else {
        println!("Invalid utf8 in output");
        std::process::exit(5);
    };

    for (idx, z) in tasks.lines().zip_longest(program_out.lines()).enumerate() {
        use itertools::EitherOrBoth as E;
        match z {
            E::Both(task, out) => {
                let Some((from, to, distance)) = task.split(";").collect_tuple() else {
                    println!("task file line={idx} of invalid format: {task:?}");
                    std::process::exit(1);
                };
                let Ok(distance) = distance.parse::<usize>() else {
                    println!("[{idx}] distance {distance:?} is no number");
                    std::process::exit(1);
                };
                let way = out.split(";").collect_vec();
                if way.first() != Some(&from) || way.last() != Some(&to) {
                    println!("[{idx}] borders not included in task={task:?} ({out:?})");
                    std::process::exit(8);
                }
                /*
                if way.len() < distance + 1 {
                    println!("[{idx}] way too short task={task:?} ({out:?})");
                    std::process::exit(9);
                }*/
                if way.len() > distance + 1 {
                    println!("[{idx}] way too long task={task:?} ({out:?})");
                    std::process::exit(9);
                }
                if let Err(err) = validate_steps::<InsertReplaceDelete, N>(list, lookup, &way) {
                    println!("[{idx}] invalid steps in task={task:?}, err={err:?} (way={way:?})");
                    std::process::exit(10);
                }
                println!("{} {out}", out.chars().filter(|c| *c == ';').count());
            }
            E::Left(task) => {
                println!("program didn't output line {idx} for task={task:?}");
                std::process::exit(6);
            }
            E::Right(out) => {
                println!("program output too long: extra line is {out:?}");
                std::process::exit(7);
            }
        }
    }
    println!("Succeeded for {} tasks", tasks.lines().count());
    println!("OK");
}
