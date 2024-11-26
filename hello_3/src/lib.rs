use std::io::{self, Write};

/// Writes “Hello, world!” to `w`.
/// 
/// # Errors
/// 
/// Returns any errors from [`writeln!`].
pub fn print(mut w: impl Write) -> io::Result<()> {
    writeln!(w, "Hello, world!")
}

#[test]
fn print_writes_message_to_writer() {
    let mut buf = Vec::new();
    print(&mut buf).unwrap();
    assert_eq!(String::from_utf8_lossy(&buf), "Hello, world!\n");
}
