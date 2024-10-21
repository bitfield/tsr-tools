use std::env;
use std::fs::{self, File};
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        if fs::exists("logbook.txt")? {
            let text = fs::read_to_string("logbook.txt")?;
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        let mut logbook = File::options()
            .create(true)
            .append(true)
            .open("logbook.txt")?;
        writeln!(logbook, "{}", args.join(" "))?;
    }
    Ok(())
}
