use anyhow::Context;

use std::{env, fs::File};

use counter_5::count_lines;

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).context("Usage: count <FILE>")?;
    let file = File::open(&path)?;
    let lines = count_lines(file)?;
    println!("{lines}");
    Ok(())
}
