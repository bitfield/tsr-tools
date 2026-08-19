use anyhow::{Context as _, Result};

use std::{env, fs::File, io::BufReader};

use count_5::count_lines;

fn main() -> Result<()> {
    let path = env::args().nth(1).context("Usage: count <FILE>")?;
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines = count_lines(reader)?;
    println!("{lines} lines");
    Ok(())
}
