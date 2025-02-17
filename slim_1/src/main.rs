use anyhow::Result;
use clap::Parser;

use slim::slim;
use slim_1 as slim;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
struct Args {
    #[arg(default_value = ".")]
    /// Path to search for Rust projects.
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = slim(args.path)?;
    print!("{output}");
    Ok(())
}
