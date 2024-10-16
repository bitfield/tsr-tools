use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<Option<String>> {
    if fs::exists(&path)? {
        let text = fs::read_to_string(&path)?;
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}

pub fn append(path: impl AsRef<Path>, msg: &str) -> io::Result<()> {
    let mut logbook = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(logbook, "{msg}")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_to_string_fn_reads_contents_of_file_as_string() {
        let text = read_to_string("tests/data/logbook.txt").unwrap();
        assert_eq!(text, Some(String::from("hello world\n")));
    }

    #[test]
    fn read_to_string_fn_returns_none_for_empty_file() {
        let text = read_to_string("tests/data/empty.txt").unwrap();
        assert_eq!(text, None);
    }

    #[test]
    fn read_to_string_fn_returns_none_if_file_does_not_exist() {
        let text = read_to_string("tests/data/bogus.txt").unwrap();
        assert_eq!(text, None);
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello logbook\n");
    }

    #[test]
    fn append_appends_line_to_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.txt");
        fs::write(&path, "hello\n").unwrap();
        append(&path, "logbook").unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "hello\nlogbook\n");
    }
}
