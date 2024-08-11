use std::io;

use hello_3::print;

fn main() -> io::Result<()> {
    print(io::stdout())
}
