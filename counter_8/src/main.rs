use std::env;

use anyhow::anyhow;
use counter_8::count_in_path;

fn main() -> anyhow::Result<()> {
    let mut word_mode = false;
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: count [-w] <FILE>..."));
    }
    for path in args {
        if path == "-w" {
            word_mode = true;
            continue;
        }
        let count = count_in_path(&path)?;
        println!(
            "{path}: {}",
            if word_mode { count.words } else { count.lines }
        );
    }
    Ok(())
}
