use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub struct Memos {
    pub path: PathBuf,
    pub inner: Vec<String>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            for memo in file.lines() {
                memos.inner.push(memo?);
            }
        }
        Ok(memos)
    }

    pub fn sync(&self) -> io::Result<()> {
        fs::write(&self.path, self.inner.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = Memos::open("tests/data/memos.txt").unwrap();
        assert_eq!(memos.inner, vec!["foo", "bar"], "wrong data");
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = Memos::open("bogus.txt").unwrap();
        assert!(memos.inner.is_empty(), "vec not empty");
    }

    #[test]
    fn sync_writes_vec_to_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        let memos = Memos {
            path: path.clone(),
            inner: vec!["foo".to_string(), "bar".to_string()],
        };
        memos.sync().unwrap();
        let memos = Memos::open(&path).unwrap();
        assert_eq!(
            memos.inner,
            vec!["foo".to_string(), "bar".to_string()],
            "wrong data"
        );
    }
}
