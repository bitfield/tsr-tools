use std::io::{BufRead, Error, Result};

/// Counts lines in `input`.
///
/// # Errors
///
/// Returns any error from [`BufRead::lines`].
pub fn count_lines(input: impl BufRead) -> Result<usize> {
    let mut count: usize = 0;
    for line in input.lines() {
        line?;
        count = count.checked_add(1).ok_or(Error::other("overflow"))?;
    }
    Ok(count)
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "test")]
#[expect(clippy::expect_used, reason = "test")]
#[expect(clippy::arbitrary_source_item_ordering, reason = "logical order")]
mod tests {
    use std::io::{BufReader, Cursor, Error, Read};

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Err(Error::other("oh no"))
        }
    }

    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        result.expect_err("no error returned");
    }
}
