use std::io::{stdin, BufRead};

fn main() {
    let input = stdin().lock();
    let lines = input.lines().count();
    println!("{lines}");
}
