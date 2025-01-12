use clap::Parser;
use walkdir::WalkDir;

use std::process::Command;

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

fn main() {
    let args = Args::parse();
    let mut targets = Vec::new();
    for path in &args.paths {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !e.path().ends_with("target/package"))
        {
            let entry = entry.unwrap();
            if entry.file_name() == "Cargo.toml" {
                let target = entry.path().to_owned();
                targets.push(target);
            };
        }
    }
    for target in targets {
        let mut cmd = Command::new("cargo");
        cmd.args(["clean", "--manifest-path", target.to_str().unwrap()]);
        if args.dry_run {
            cmd.arg("--dry-run");
        }
        let output = cmd.output().unwrap();
        println!(
            "{}: {}",
            target.parent().unwrap().display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
}
