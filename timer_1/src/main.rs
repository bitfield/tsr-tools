use anyhow::Result;
use clap::Parser;

use std::{process::Command, time::Instant};

#[derive(Parser)]
/// Runs a given command and reports elapsed time.
struct Args {
    /// Arguments to the program.
    args: Vec<String>,
    /// Name or path of the program to run.
    #[arg(required = true)]
    program: String,
}

#[expect(clippy::use_debug, reason = "debug formatting is okay")]
fn main() -> Result<()> {
    let args = Args::parse();
    let mut cmd = Command::new(args.program);
    cmd.args(args.args);
    let start = Instant::now();
    let output = cmd.output()?;
    let elapsed = start.elapsed();
    print!("{}", String::from_utf8_lossy(&output.stdout));
    print!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{elapsed:.1?}");
    Ok(())
}
