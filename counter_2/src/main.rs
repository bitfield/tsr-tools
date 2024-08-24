use std::io;

use counter_2::count_lines;

fn main() {
    let lines = count_lines(io::stdin());
    println!("{lines}");
}
