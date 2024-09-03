use std::env;

use anyhow::anyhow;
use counter_7::count_lines_in_path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: count <FILE>..."));
    }
    for path in args {
        println!("{path}: {}", count_lines_in_path(&path)?);
    }
    Ok(())
}