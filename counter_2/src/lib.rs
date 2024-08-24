use std::io::{BufRead, BufReader, Read};

pub fn count_lines(input: impl Read) -> usize {
    let reader = BufReader::new(input);
    reader.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn count_lines_fn_counts_lines_in_input() {
        let input = io::Cursor::new("line 1\nline2\n");
        let lines = count_lines(input);
        assert_eq!(2, lines);
    }
}
// 