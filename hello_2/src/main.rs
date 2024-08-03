use hello_2 as hello;

fn main() -> std::io::Result<()> {
    hello::print(std::io::stdout())
}
