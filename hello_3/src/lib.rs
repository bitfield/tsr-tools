#[must_use]
pub fn world() -> String {
    String::from("Hello, world!")
}

#[test]
fn world_returns_hello_world() {
    assert_eq!(world(), "Hello, world!", "wrong message");
}
