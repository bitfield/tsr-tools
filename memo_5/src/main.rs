use anyhow::Result;
use clap::Parser;

use memo_5 as memo;
use memo::{Memo, Memos, Status};

#[derive(Parser)]
/// Stores and manages simple reminders.
struct Args {
    /// Marks all matching memos as done
    #[arg(short, long)]
    done: bool,
    /// Deletes all memos with status “done”
    #[arg(short, long)]
    purge: bool,
    /// Text of the memo to store or mark as done
    text: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut memos = Memos::open("memos.json")?;
    let text = args.text.join(" ");
    if args.purge {
        memos.purge_done();
        memos.sync()?;
    }
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
