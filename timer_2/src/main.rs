use anyhow::Result;
use clap::Parser;

use timer_2 as timer;

#[derive(Parser)]
/// Runs a given command and reports elapsed time.
struct Args {
    /// Arguments to the program.
    args: Vec<String>,
    #[arg(required = true)]
    /// Name or path of the program to run.
    program: String,
}

#[expect(clippy::use_debug, reason = "debug formatting is okay")]
fn main() -> Result<()> {
    let args = Args::parse();
    let report = timer::time(&args.program, &args.args)?;
    print!("{}", report.stdout);
    print!("{}", report.stderr);
    println!("{:.1?}", report.elapsed);
    Ok(())
}
