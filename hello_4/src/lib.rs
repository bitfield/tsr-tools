#[must_use]
pub fn world() -> String {
    String::from("Hello, world!")
}

#[test]
fn world_returns_expected_string() {
    assert_eq!("Hello, world!", world());
}
