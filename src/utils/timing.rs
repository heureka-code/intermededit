use std::time::Instant;

pub fn time_function<T>(label: &str, func: impl FnOnce() -> T) -> T {
    eprintln!("Start: {label}");
    let start = Instant::now();
    let value = func();
    eprintln!("Finished in {:?}: {label}", start.elapsed());
    value
}
