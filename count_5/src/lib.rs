use anyhow::Result;

use std::io::BufRead;

/// Counts lines in `input`.
///
/// # Errors
///
/// Returns any error from [`BufRead::lines`].
pub fn count_lines(input: impl BufRead) -> Result<usize> {
    let mut count = 0;
    for line_res in input.lines() {
        line_res?;
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufReader, Cursor, Error, ErrorKind, Read};

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "oh no"))
        }
    }

    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err());
    }
}
