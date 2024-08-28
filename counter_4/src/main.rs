use std::{env, fs::File, io::BufReader};

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
    let lines = count_lines(BufReader::new(file))
        .map_err(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
        .unwrap();
    println!("{lines}");
}
