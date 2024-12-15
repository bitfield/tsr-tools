use anyhow::{Context, Result};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

/// Counts words and lines in the given reader.
///
/// # Errors
///
/// Returns any error from [`BufReader::read_line`].
pub fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();
    loop {
        let bytes_read = input.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

/// Counts words and lines in the file at `path`.
///
/// # Errors
///
/// Returns any error from [`File::open`] or [`count`].
pub fn count_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);
    count(file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufReader, Cursor, Error, ErrorKind, Read};

    use super::*;

    #[test]
    fn count_counts_lines_and_words_in_input() {
        let input = Cursor::new("word1 word2\nword3");
        let count = count(input).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 3, "wrong word count");
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "oh no"))
        }
    }

    #[test]
    fn count_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);
        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_in_path_fn_counts_lines_and_words_in_given_file() {
        let path = String::from("tests/data/test.txt");
        let count = count_in_path(&path).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 4, "wrong word count");
    }
}
