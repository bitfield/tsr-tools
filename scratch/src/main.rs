use anyhow::anyhow;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

// logbook 1
// fn main() -> anyhow::Result<()> {
//     let mut logbook = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("logbook.txt")?;
//     writeln!(logbook, "Hello, logbook!")?;
//     Ok(())
// }

// logbook 2
fn main() -> anyhow::Result<()> {
    let line = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if line.is_empty() {
        return Err(anyhow!("Usage: logbook <LINE>"));
    }

    let mut logbook = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "{line}")?;
    Ok(())
}
