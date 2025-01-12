use std::time::Instant;

fn main() {
    let start = Instant::now();
    for _ in 1..=100 {
        let _ = 2 + 2;
    }
    println!("That took {:?}", start.elapsed());
}

// fn main() -> Result<()> {
//     let mut cmd = Command::new("echo");
//     cmd.args(["hello", "world"]);
//     let output = cmd.output()?;
//     let stdout = String::from_utf8(output.stdout)?;
//     println!("output: {stdout}");
//     Ok(())
// }
