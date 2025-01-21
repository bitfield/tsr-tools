use anyhow::{bail, Result};

use std::env;

use count_8::count_in_path;

fn main() -> Result<()> {
    let mut word_mode = false;
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count [-w] <FILE>...");
    }
    for path in args {
        if path == "-w" {
            word_mode = true;
            continue;
        }
        let count = count_in_path(&path)?;
        if word_mode {
            println!("{path}: {} words", count.words);
        } else {
            println!("{path}: {} lines", count.lines);
        };
    }
    Ok(())
}
