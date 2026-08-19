use std::{io::stdin, process};

use count_3::count_lines;

fn main() {
    let res = count_lines(stdin().lock());
    match res {
        Ok(lines) => println!("{lines} lines"),
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
