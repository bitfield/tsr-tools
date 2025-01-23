//! Records observations in a logbook file, or lists previous
//! observations.
use anyhow::Result;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Reads the contents of the logbook file at `path`.
///
/// Returns [`None`] if the file does not exist or is empty.
///
/// # Errors
///
/// Returns any error from [`fs::exists`] or [`fs::read_to_string`].
pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
    if fs::exists(&path)? {
        let text = fs::read_to_string(path)?;
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}

/// Appends `msg` to the logbook file at `path`, creating the file if necessary.
///
/// # Errors
///
/// Returns any error from [`open`](fs::OpenOptions::open) or [`writeln!`].
pub fn append(path: impl AsRef<Path>, msg: &str) -> Result<()> {
    let mut logbook =
        File::options().create(true).append(true).open(path)?;
    writeln!(logbook, "{msg}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_reads_contents_of_file_as_string() {
        let text =
            read("tests/data/logbook.txt").unwrap().unwrap();
        assert_eq!(text.trim_end(), "hello world", "wrong text");
    }

    #[test]
    fn read_returns_none_for_empty_file() {
        let text = read("tests/data/empty.txt").unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let text = read("tests/data/bogus.txt").unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello logbook\n", "wrong text");
    }

    #[test]
    fn append_appends_line_to_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.txt");
        fs::write(&path, "hello\n").unwrap();
        append(&path, "logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello\nlogbook\n", "wrong text");
    }
}
