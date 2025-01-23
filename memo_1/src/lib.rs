use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Result},
    path::Path,
};

/// Reads the contents of the memo file at `path`.
///
/// Returns an empty [`Vec`] if the file does not exist or is empty.
///
/// # Errors
///
/// Returns any error from [`fs::exists`], [`File::open`], or [`BufRead::lines`].
pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    if fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

/// Writes `memos` to the file at `path`, creating it if necessary.
///
/// # Errors
///
/// Returns any error from [`fs::write`].
pub fn sync(
    memos: &[String],
    path: impl AsRef<Path>,
) -> Result<()> {
    fs::write(&path, memos.join("\n"))
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = open("tests/data/memos.txt").unwrap();
        assert_eq!(memos, vec!["foo", "bar"], "wrong data");
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = open("bogus.txt").unwrap();
        assert!(memos.is_empty(), "vec not empty");
    }

    #[test]
    fn sync_writes_vec_to_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        let vec = vec!["foo".to_string(), "bar".to_string()];
        sync(&vec, &path).unwrap();
        let memos = open(&path).unwrap();
        assert_eq!(memos, vec, "wrong data");
    }
}
