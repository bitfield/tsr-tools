use anyhow::Result;
use clap::Parser;

use weather::Weatherstack;
use weather_3 as weather;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    #[arg(
        short,
        long,
        env = "WEATHERSTACK_API_KEY",
        required = true
    )]
    /// Weatherstack API key
    api_key: String,
    #[arg(required = true)]
    /// Example: "London,UK"
    location: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let location = args.location.join(" ");
    let ws = Weatherstack::new(&args.api_key);
    let weather = ws.get_weather(&location)?;
    println!("{weather}");
    Ok(())
}
