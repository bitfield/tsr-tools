use anyhow::Result;
use clap::Parser;

use scratch as timer;

#[derive(Parser)]
/// Runs a given command and reports elapsed time.
struct Args {
    #[arg(required = true)]
    /// Name or path of the program to run
    program: String,
    /// Arguments to the program
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let report = timer::time(&args.program, &args.args)?;
    print!("{}", report.stdout);
    print!("{}", report.stderr);
    println!(
        "`{} {}`: {:.1?}",
        args.program,
        args.args.join(" "),
        report.elapsed
    );
    Ok(())
}
