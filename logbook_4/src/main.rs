use logbook_4 as logbook;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        if let Some(text) = logbook::read_to_string("logbook.txt")? {
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        logbook::append("logbook.txt", &args.join(" "))?;
    }
    Ok(())
}
