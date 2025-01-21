use anyhow::{Context, Result};

use std::{env, fs::File, io::BufReader};

use count_5::count_lines;

fn main() -> Result<()> {
    let path = env::args().nth(1).context("Usage: count <FILE>")?;
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    let lines = count_lines(file)?;
    println!("{lines} lines");
    Ok(())
}
