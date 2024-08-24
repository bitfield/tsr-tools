use std::io::{self, BufRead, BufReader};

#[must_use]
pub fn count_lines_stdin() -> usize {
    Counter::new().count_lines()
}

pub struct Counter {
    pub input: Box<dyn BufRead>,
}

impl Counter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            input: Box::new(BufReader::new(io::stdin())),
        }
    }

    #[must_use]
    pub fn count_lines(self) -> usize {
        self.input.lines().count()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines_fn_returns_expected_result() {
        let mut ctr = Counter::new();
        ctr.input = Box::new(io::Cursor::new(b"line 1\nline2\n"));
        let lines = ctr.count_lines();
        assert_eq!(2, lines, "want 2 lines, got {lines}");
    }
}
