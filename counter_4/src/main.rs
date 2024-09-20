use std::{env, fs::File};

use counter_4::count_lines;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage: count <FILE>");
        std::process::exit(1);
    };
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
