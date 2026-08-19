use anyhow::Result;

use std::{
    fmt::{Display, Formatter},
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Memos {
    pub inner: Vec<Memo>,
    pub path: PathBuf,
}

impl Memos {
    /// Reads the contents of the memo file at `path`.
    ///
    /// Returns an empty [`Memos`] if the file does not exist or is empty.
    ///
    /// # Errors
    ///
    /// Returns any error from [`fs::exists`], [`File::open`], or
    /// [`serde_json::from_reader`].
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = File::open(path)?;
            memos.inner = serde_json::from_reader(BufReader::new(file))?;
        }
        Ok(memos)
    }

    /// Writes `memos` to the file at `path`, creating it if necessary.
    ///
    /// # Errors
    ///
    /// Returns any error from [`File::create`] or [`serde_json::to_writer`].
    pub fn sync(&self) -> Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Memo {
    pub status: Status,
    pub text: String,
}

impl Display for Memo {
    #[expect(clippy::absolute_paths, reason = "not anyhow::Result")]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Done,
    Pending,
}

impl Display for Status {
    #[expect(clippy::absolute_paths, reason = "not anyhow::Result")]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Pending => "-",
                Self::Done => "x",
            }
        )
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests")]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = Memos::open("bogus.json").unwrap();
        assert!(memos.inner.is_empty(), "vec not empty");
    }

    #[test]
    fn round_trip_via_sync_and_open_preserves_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.json");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_owned(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_owned(),
                    status: Status::Pending,
                },
            ],
        };
        memos.sync().unwrap();
        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "wrong data");
    }
}
