use std::fs::OpenOptions;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let mut logbook = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "Hello, logbook!")?;
    Ok(())
}

