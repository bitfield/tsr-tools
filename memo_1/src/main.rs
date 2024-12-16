use anyhow::Result;

use std::env;

use memo::{open, sync};
use memo_1 as memo;

fn main() -> Result<()> {
    let mut memos = open("memos.txt")?;
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        for memo in &memos {
            println!("{memo}");
        }
    } else {
        let memo = args.join(" ");
        memos.push(memo);
        sync(&memos, "memos.txt")?;
    }
    Ok(())
}
