use std::io::stdin;

use count_2::count_lines;

fn main() {
    let lines = count_lines(stdin().lock());
    println!("{lines}");
}
