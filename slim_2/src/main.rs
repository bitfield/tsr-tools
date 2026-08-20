use anyhow::Result;
use clap::Parser;

use slim::Slimmer;
use slim_2 as slim;

#[derive(Parser)]
/// Runs `cargo clean` recursively to save disk space by deleting build
/// artifacts.
struct Args {
    /// Don't delete anything, just show what would happen.
    #[arg(long)]
    dry_run: bool,
    /// Paths to search for Rust projects.
    #[arg(default_value = ".")]
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
