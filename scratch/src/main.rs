use anyhow::Result;
use clap::Parser;

use scratch as weather;
use weather::get_weather;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
    /// Weatherstack API key
    api_key: String,
    #[arg(required = true)]
    /// Example: "London,UK"
    location: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let location = args.location.join(" ");
    println!("{location} {}", args.api_key);
    let weather = get_weather(&location, &args.api_key)?;
    println!("{weather}");
    Ok(())
}
