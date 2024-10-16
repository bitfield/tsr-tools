use std::io::{self, BufRead, BufReader};

pub fn count_lines(input: impl io::Read) -> io::Result<usize> {
    let reader = BufReader::new(input);
    let mut count = 0;
    for line_res in reader.lines() {
        line_res?;
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use std::io::{self, ErrorKind};

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = io::Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2);
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
}
