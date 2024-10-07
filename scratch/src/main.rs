use std::fs;
use std::io::Write;
use std::{env, fs::OpenOptions};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        if fs::exists("logbook.txt")? {
            let text = fs::read_to_string("logbook.txt")?;
            println!("{text}");
        } else {
            println!("Logbook is empty");
        }
    }
    let mut logbook = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logbook.txt")?;
    writeln!(logbook, "{}", args.join(" "))?;
    Ok(())
}
