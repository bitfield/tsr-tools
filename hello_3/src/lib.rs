#[must_use]
pub fn world() -> String {
    String::from("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_fn_returns_expected_string() {
        assert_eq!(String::from("Hello, world!"), world());
    }
}