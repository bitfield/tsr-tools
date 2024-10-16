use std::{env, fs::OpenOptions};
use anyhow::anyhow;
use std::io::Write;
fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: logbook <LINE>"));
    }
    let mut logbook = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "{}", &args.join(" "))?;
    Ok(())
}