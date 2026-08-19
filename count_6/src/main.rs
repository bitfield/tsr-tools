use anyhow::{Context as _, Result};

use std::{env, fs::File, io::BufReader};

use count_6::count_lines;

fn main() -> Result<()> {
    for path in env::args().skip(1) {
        let file = File::open(&path).with_context(|| path.clone())?;
        let reader = BufReader::new(file);
        let lines = count_lines(reader).with_context(|| path.clone())?;
        println!("{path}: {lines} lines");
    }
    Ok(())
}
