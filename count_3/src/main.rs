use std::io::stdin;
use std::process;

use count_3::count_lines;

fn main() {
    let res = count_lines(stdin().lock());
    match res {
        Ok(lines) => println!("{lines}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
