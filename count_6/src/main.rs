use anyhow::Context;

use std::{env, fs::File, io::BufReader};

use count_6::count_lines;

fn main() -> anyhow::Result<()> {
    for path in env::args().skip(1) {
        let file = File::open(&path).with_context(|| path.clone())?;
        let file = BufReader::new(file);
        let lines = count_lines(file).with_context(|| path.clone())?;
        println!("{path}: {lines}");
    }
    Ok(())
}
