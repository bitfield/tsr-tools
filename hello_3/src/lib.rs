use std::io::Write;

pub fn print(mut w: impl Write) {
    writeln!(w, "Hello, world!");
}

#[test]
fn print_writes_message_to_writer() {
    let mut buf = Vec::new();
    print(&mut buf);
    assert_eq!("Hello, world!\n", String::from_utf8_lossy(&buf));
}
