use std::fs::File;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let mut logbook = File::options()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "Hello, logbook!")?;
    Ok(())
}
