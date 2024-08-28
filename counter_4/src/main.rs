use std::{env, fs::File};

use counter_4::count_lines;

fn main() {
    let path = env::args()
        .nth(1)
        .or_else(|| {
            eprintln!("Usage: count <FILE>");
            std::process::exit(1);
        })
        .unwrap();
    let file = File::open(&path)
        .map_err(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
        .unwrap();
    let lines = count_lines(file)
        .map_err(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
        .unwrap();
    println!("{lines}");
}
