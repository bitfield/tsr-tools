use std::env;

use logbook_4 as logbook;

fn main() -> anyhow::Result<()> {
    let line = env::args().skip(1).collect::<Vec<_>>().join(" ");

    if line.is_empty() {
        if let Some(text) = logbook::read_to_string("logbook.txt")? {
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        logbook::append("logbook.txt", &line)?;
    }
    Ok(())
}
