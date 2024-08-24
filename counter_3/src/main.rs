use std::io;

use counter_3::count_lines;

fn main() {
    let res = count_lines(io::stdin());
    match res {
        Ok(lines) => println!("{lines}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
