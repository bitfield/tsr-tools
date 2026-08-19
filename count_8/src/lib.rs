use anyhow::{Context as _, Result};

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug, Default)]
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
    while input.read_line(&mut line)? > 0 {
        count.lines =
            count.lines.checked_add(1).ok_or(Error::other("overflow"))?;
        count.words = count
            .words
            .checked_add(line.split_whitespace().count())
            .ok_or(Error::other("overflow"))?;
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
    let reader = BufReader::new(file);
    count(reader).with_context(|| path.clone())
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "test")]
#[expect(clippy::expect_used, reason = "test")]
#[expect(clippy::arbitrary_source_item_ordering, reason = "logical order")]
mod tests {
    use std::io::{self, BufReader, Cursor, Error, Read};

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
            Err(Error::other("oh no"))
        }
    }

    #[test]
    fn count_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);
        result.expect_err("no error returned");
    }

    #[test]
    fn count_in_path_fn_counts_lines_and_words_in_given_file() {
        let path = String::from("tests/data/test.txt");
        let count = count_in_path(&path).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 4, "wrong word count");
    }
}
