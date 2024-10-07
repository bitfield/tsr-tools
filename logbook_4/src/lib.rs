use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};


pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) if text.is_empty() => Ok(None),
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn append(path: impl AsRef<Path>, line: &str) -> io::Result<()> {
    let mut logbook = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(logbook, "{line}")
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_to_string_fn_reads_contents_of_file_as_string() {
        let text = read_to_string("tests/testdata/logbook.txt").unwrap();
        assert_eq!(Some("hello world\n".into()), text);
    }

    #[test]
    fn read_to_string_fn_returns_none_for_empty_file() {
        let text = read_to_string("tests/testdata/empty_logbook.txt").unwrap();
        assert_eq!(None, text);
    }

    #[test]
    fn read_to_string_fn_returns_none_if_file_does_not_exist() {
        let text = read_to_string("tests/testdata/doesnotexist.txt").unwrap();
        assert_eq!(None, text);
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.txt");
        append(&path, "hello logbook").unwrap();
        let text = std::fs::read_to_string(path).unwrap();
        assert_eq!("hello logbook\n", text);
    }
    
    #[test]
    fn append_appends_line_to_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.txt");
        fs::write(&path, "hello\n").unwrap();
        println!("{path:?}");
        append(&path, "logbook").unwrap();
        let text = std::fs::read_to_string(path).unwrap();
        assert_eq!("hello\nlogbook\n", text);
    }
}
