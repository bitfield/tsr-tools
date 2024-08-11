use std::io::{self, Write};

pub fn print(mut w: impl Write) -> io::Result<()> {
    writeln!(w, "Hello, world!")
}

#[test]
fn print_fn_writes_given_text_to_writer() {
    let mut buf = Vec::new();
    print(&mut buf).expect("write should succeed");
    assert_eq!("Hello, world!\n", String::from_utf8_lossy(&buf));
}
