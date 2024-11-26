use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Memos {
    pub path: PathBuf,
    pub inner: Vec<Memo>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
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

    pub fn sync(&self) -> io::Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Pending,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pending => "-",
                Self::Done => "x",
            }
        )
    }
}

#[cfg(test)]
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
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };
        memos.sync().unwrap();
        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "wrong data");
    }
}
