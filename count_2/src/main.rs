use std::io;

use count_2::count_lines;

fn main() {
    let lines = count_lines(io::stdin().lock());
    println!("{lines}");
}
