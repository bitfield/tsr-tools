use anyhow::Result;
use clap::Parser;

use std::{process::Command, time::Instant};

#[derive(Parser)]
/// Runs a given command and reports elapsed time.
struct Args {
    #[arg(required = true)]
    /// Name or path of the program to run
    program: String,
    /// Arguments to the program
    args: Vec<String>,
}

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
