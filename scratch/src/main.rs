// counter_7
// use std::env;

// use anyhow::{anyhow, Context};
// use scratch::count_lines_in_path;

// fn main() -> anyhow::Result<()> {
//     let args: Vec<_> = env::args().skip(1).collect();
//     if args.is_empty() {
//         return Err(anyhow!("Usage: count <FILE>..."));
//     }
//     for path in args {
//         let lines = count_lines_in_path(&path).with_context(|| path.clone())?;
//         println!("{path}: {lines}");
//     }
//     Ok(())
// }
//
// pub fn count_lines_in_path(path: &str) -> io::Result<usize> {
//     let file = File::open(path)?;
//     count_lines(BufReader::new(file))
// }
//
// #[test]
// fn count_lines_in_path_fn_returns_expected_result() {
//     let lines = count_lines_in_path(&"tests/data/test.txt".to_string()).unwrap();
//     assert_eq!(2, lines);
// }

use std::{env, fs::File};

use anyhow::{anyhow, Context};
use scratch::{count_lines, count_lines_in_path};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!("Usage: count <FILE>..."));
    }
    for path in args {
        println!("{path}: {}", count_lines_in_path(&path)?);
    }
    Ok(())
}
