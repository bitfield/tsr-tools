use anyhow::Result;
use clap::Parser;

use slim::slim;
use slim_1 as slim;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
struct Args {
    /// Path to search for Rust projects.
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = slim(args.path)?;
    print!("{output}");
    Ok(())
}
