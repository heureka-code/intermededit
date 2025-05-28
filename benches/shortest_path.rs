use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};

use intermededit::{Word, find_way, read_wordlist};

fn criterion_benchmark(c: &mut Criterion) {
    let all_words = read_wordlist("wordlist-german.txt").expect("Wordlist file to be present");
    let mut g = c.benchmark_group("shortest-path");
    g.measurement_time(Duration::from_secs(30));

    let mut default_bench = |start: &str, target: &str| {
        g.bench_function(
            format!("{} -> {}", start.to_uppercase(), target.to_uppercase()),
            |b| {
                b.iter(|| {
                    let _ = black_box(find_way(
                        &all_words,
                        Word::new(start),
                        10,
                        Word::new(target),
                    ));
                });
            },
        );
    };
    default_bench("Herz", "rasen");
    default_bench("Bier", "Leber");
    default_bench("blau", "Alge");
    default_bench("Rhein", "raus");
    default_bench("Eis", "kalt");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
