use intermededit::AllWords;

fn print_len_histogram(by_length: &AllWords) {
    for (ind, h) in by_length.iter_lengths().enumerate() {
        println!(
            "{ind:02} {} on {}",
            h.values().flatten().count(),
            h.values().count()
        );
    }
}
