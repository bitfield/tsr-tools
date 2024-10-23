use std::io::BufRead;

pub fn count_lines(input: impl BufRead) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input);
        assert_eq!(lines, 2, "wrong line count");
    }
}
