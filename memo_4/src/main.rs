use anyhow::Result;
use clap::Parser;

use memo::{Memo, Memos, Status};
use memo_4 as memo;

#[derive(Parser)]
/// Stores and manage simple reminders.
struct Args {
    /// Marks all matching memos as done
    #[arg(short, long)]
    done: bool,
    /// Text of the memo to store or mark as done
    text: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut memos = Memos::open("memos.json")?;
    let text = args.text.join(" ");
    if args.done {
        for m in memos.find_all(&text) {
            m.status = Status::Done;
            println!("Marked \"{}\" as done.", m.text);
        }
        memos.sync()?;
    } else if args.text.is_empty() {
        for memo in &memos.inner {
            println!("{memo}");
        }
    } else {
        memos.inner.push(Memo {
            text: text.clone(),
            status: Status::Pending,
        });
        println!("Added \"{}\" as a new memo.", &text);
        memos.sync()?;
    }
    Ok(())
}
