use anyhow::Context;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn count_lines(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;
    for line_res in input.lines() {
        line_res?;
        count += 1;
    }
    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> anyhow::Result<usize> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);
    count_lines(file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind, Read};

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = io::Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "oh no"))
        }
    }

    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err());
    }

    #[test]
    fn count_lines_in_path_fn_counts_lines_in_given_file() {
        let path = String::from("tests/data/test.txt");
        let lines = count_lines_in_path(&path).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }
}
