use anyhow::Result;
use clap::Parser;

use slim::Slimmer;
use slim_2 as slim;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
struct Args {
    #[arg(long)]
    /// Don't delete anything, just show what would happen.
    dry_run: bool,
    #[arg(default_value = ".")]
    /// Paths to search for Rust projects.
    paths: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut slimmer = Slimmer::new();
    if args.dry_run {
        slimmer.dry_run = true;
    }
    for path in &args.paths {
        let output = slimmer.slim(path)?;
        print!("{output}");
    }
    Ok(())
}
