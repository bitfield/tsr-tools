use anyhow::Result;
use clap::Parser;

use memo::{Memo, Memos, Status};
use memo_4 as memo;

#[derive(Parser)]
/// Stores and manages simple reminders.
struct Args {
    /// Marks all matching memos as done.
    #[arg(short, long)]
    done: bool,
    /// Text of the memo to store or mark as done.
    text: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut memos = Memos::open("memos.json")?;
    let text = args.text.join(" ");
    if args.done {
        for memo in memos.find_all(&text) {
            memo.status = Status::Done;
            println!("Marked \"{}\" as done.", memo.text);
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
        println!("Added \"{text}\" as a new memo.");
        memos.sync()?;
    }
    Ok(())
}
