use std::{env, fs::File, io::BufReader, process};

use count_4::count_lines;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage: count <FILE>");
        process::exit(1);
    };
    let file = File::open(&path)
        .map_err(|err| {
            eprintln!("{err}");
            process::exit(1);
        })
        .unwrap();
    let reader = BufReader::new(file);
    let lines = count_lines(reader)
        .map_err(|err| {
            eprintln!("{err}");
            process::exit(1);
        })
        .unwrap();
    println!("{lines} lines");
}
