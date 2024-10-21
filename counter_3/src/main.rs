use std::{io, process};

use counter_3::count_lines;

fn main() {
    let res = count_lines(io::stdin().lock());
    match res {
        Ok(lines) => println!("{lines}"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
