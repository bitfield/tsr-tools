use std::io::{self, BufRead};

fn main() {
    let input = io::stdin().lock();
    let lines = input.lines().count();
    println!("{lines}");
}
