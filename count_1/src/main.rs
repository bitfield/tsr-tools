use std::io::{BufRead as _, stdin};

fn main() {
    let input = stdin().lock();
    let lines = input.lines().count();
    println!("{lines} lines");
}
