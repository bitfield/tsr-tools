use anyhow::Result;
use walkdir::WalkDir;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn slim(path: impl AsRef<Path>) -> Result<String> {
    Ok(String::new())
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

fn cargo_clean_cmd(path: impl AsRef<Path>) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.args([
        "clean",
        "--manifest-path",
        &path.as_ref().to_string_lossy(),
    ]);
    cmd
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, process::Command};

    use super::*;

    #[test]
    fn manifests_returns_cargo_toml_paths() {
        let manifests = manifests("tests/data").unwrap();
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
        let cmd = cargo_clean_cmd(PathBuf::from("code/proj_1/Cargo.toml"));
        assert_eq!(cmd.get_program(), "cargo", "wrong program");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            ["clean", "--manifest-path", "code/proj_1/Cargo.toml"],
            "wrong args"
        );
    }
}
