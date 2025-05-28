use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};

use intermededit::{Word, read_wordlist};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("wordlist");
    g.measurement_time(Duration::from_secs(60));

    let words = std::fs::read_to_string("wordlist-german.txt").expect("Wordlist file to be present");
    g.bench_function("bitmasks", |b| {
        b.iter(|| {
            for word in words.lines().map(Word::new) {
                black_box(word);
            }
        });
    });

    g.bench_function("io and bitmasks", |b| {
            b.iter(|| {
                let _ = read_wordlist(black_box("wordlist-german.txt")).expect("Wordlist file to be present");
            });
        });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
