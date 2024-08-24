use std::io::{self, BufRead, BufReader};

fn main() {
    let input = BufReader::new(io::stdin());
    let lines = input.lines().count();
    println!("{lines}");
}
