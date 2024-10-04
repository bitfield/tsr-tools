use std::fs;
use std::io::Write;
use std::{env, fs::OpenOptions};

fn main() -> anyhow::Result<()> {
    let line = env::args().skip(1).collect::<Vec<_>>().join(" ");

    if line.is_empty() {
        if fs::exists("logbook.txt")? {
            let text = fs::read_to_string("logbook.txt")?;
            println!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        let mut logbook = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logbook.txt")?;
        writeln!(logbook, "{line}")?;
    }
    Ok(())
}
