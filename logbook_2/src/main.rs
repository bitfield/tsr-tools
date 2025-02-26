use anyhow::{Result, bail};

use std::{env, fs::File, io::Write};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: logbook <MESSAGE>");
    }
    let mut logbook = File::options()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "{}", args.join(" "))?;
    Ok(())
}
