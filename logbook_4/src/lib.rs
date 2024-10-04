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
    use tempfile::{env::temp_dir, NamedTempFile};

    use super::*;

    #[test]
    fn read_to_string_fn_reads_contents_of_file_string() {
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
    fn append_appends_line_to_existing_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test").unwrap();

        let path = file.into_temp_path();

        append(&path, "test 2").unwrap();

        let text = std::fs::read_to_string(path).unwrap();
        assert_eq!("test\ntest 2\n", text);
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = temp_dir();
        let path = dir.join("logbook.txt");
        append(&path, "test").unwrap();

        let text = std::fs::read_to_string(path).unwrap();
        assert_eq!("test\n", text);
    }
}
