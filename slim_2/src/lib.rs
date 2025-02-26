use anyhow::Result;
use walkdir::WalkDir;

use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

#[derive(Default)]
pub struct Slimmer {
    pub dry_run: bool,
}

impl Slimmer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs `cargo clean` on all Rust projects in `path`.
    ///
    /// # Errors
    ///
    /// Returns any errors searching `path`, or when running Cargo.
    ///
    /// # Panics
    ///
    /// Panics if the target manifest file does not have a parent directory.
    /// This should never happen because targets are always `Cargo.toml` files.
    pub fn slim(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut output = String::new();
        for target in manifests(path)? {
            let mut cmd = self.cargo_clean_cmd(&target);
            let cmd_output = cmd.output()?;
            output.push_str(&summary(target, &cmd_output));
        }
        Ok(output)
    }

    fn cargo_clean_cmd(&self, target: impl AsRef<Path>) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "clean",
            "--manifest-path",
            &target.as_ref().to_string_lossy(),
        ]);
        if self.dry_run {
            cmd.arg("--dry-run");
        }
        cmd
    }
}

/// Finds all `Cargo.toml` files in the tree rooted at `path`.
///
/// # Errors
///
/// Returns any errors encountered by [`walkdir`].
fn manifests(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            targets.push(entry.path().to_path_buf());
        }
    }
    Ok(targets)
}

fn summary(target: impl AsRef<Path>, output: &Output) -> String {
    format!(
        "{}: {}",
        target.as_ref().parent().unwrap().display(),
        String::from_utf8_lossy(&output.stderr).trim_start()
    )
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, process::ExitStatus};

    use super::*;

    #[test]
    fn manifests_returns_cargo_toml_paths() {
        let mut manifests = manifests("tests/data").unwrap();
        manifests.sort();
        assert_eq!(
            manifests,
            vec![
                PathBuf::from("tests/data/proj_1/Cargo.toml"),
                PathBuf::from("tests/data/proj_2/Cargo.toml"),
                PathBuf::from("tests/data/proj_3/Cargo.toml"),
            ],
            "wrong paths"
        );
    }

    #[test]
    fn cargo_clean_cmd_fn_returns_correct_cargo_command() {
        let slimmer = Slimmer::new();
        let cmd = slimmer.cargo_clean_cmd(PathBuf::from(
            "code/proj_1/Cargo.toml",
        ));
        assert_eq!(cmd.get_program(), "cargo", "wrong program");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            ["clean", "--manifest-path", "code/proj_1/Cargo.toml"],
            "wrong args"
        );
    }

    #[test]
    fn cargo_clean_cmd_fn_honours_dry_run_mode() {
        let mut slimmer = Slimmer::new();
        slimmer.dry_run = true;
        let cmd = slimmer.cargo_clean_cmd("./target/Cargo.toml");
        assert_eq!(cmd.get_program(), "cargo", "wrong program");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            vec![
                "clean",
                "--manifest-path",
                "./target/Cargo.toml",
                "--dry-run"
            ],
            "wrong args"
        );
    }

    #[test]
    fn summary_reports_target_path_and_cargo_output() {
        let cmd_output = summary(
            PathBuf::from("./target/Cargo.toml"),
            &Output {
                stdout: Vec::new(),
                stderr: String::from(
                    "     Removed 2 files, 1.6MiB total\n",
                )
                .into_bytes(),
                status: ExitStatus::default(),
            },
        );
        assert_eq!(
            cmd_output, "./target: Removed 2 files, 1.6MiB total\n",
            "wrong formatting"
        );
    }
}
