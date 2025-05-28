use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};

use intermededit::{Word, generate_and_traverse_all_ways_without_stopping, read_wordlist};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("generate-ways");
    g.measurement_time(Duration::from_secs(60));
    let all_words = read_wordlist("wordlist-german.txt").expect("Wordlist file to be present");

    let mut default_bench = |word: &str, depth| {
        g.bench_function(format!("{} {depth}", word.to_uppercase()), |b| {
            b.iter(|| {
                generate_and_traverse_all_ways_without_stopping(
                    &all_words,
                    Word::new(word),
                    black_box(depth),
                );
            });
        });
    };

    default_bench("Herz", 5);
    default_bench("Eis", 5);
    default_bench("blau", 5);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
