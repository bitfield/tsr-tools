use anyhow::Context;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn count_lines(input: impl io::Read) -> io::Result<usize> {
    let mut reader = BufReader::new(input);
    let mut count = 0;
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        count += 1;
        line.clear();
    }
    Ok(count)
}

pub fn count_words(input: impl io::Read) -> io::Result<usize> {
    let mut reader = BufReader::new(input);
    let mut count = 0;
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        count += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> anyhow::Result<usize> {
    let file = File::open(path).with_context(|| path.clone())?;
    count_lines(file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, ErrorKind};

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = io::Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(2, lines);
    }

    #[test]
    fn count_words_fn_counts_words_in_input() {
        let input = io::Cursor::new("word1 word2 word3");
        let lines = count_words(input).unwrap();
        assert_eq!(3, lines);
    }

    struct ErrorReader;

    impl io::Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "oh no"))
        }
    }

    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let result = count_lines(ErrorReader);
        assert!(result.is_err());
    }

    #[test]
    fn count_in_path_fn_counts_lines_and_words_in_given_file() {
        let lines = count_lines_in_path(&"tests/data/test.txt".to_string()).unwrap();
        assert_eq!(2, lines);
    }
}
