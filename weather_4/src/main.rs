use anyhow::Result;
use clap::Parser;

use weather::Weatherstack;
use weather_4 as weather;

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
    #[arg(short, long)]
    /// Report temperatures in Fahrenheit
    fahrenheit: bool,
    #[arg(required = true)]
    /// Example: "London,UK"
    location: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let location = args.location.join(" ");
    let ws = Weatherstack::new(&args.api_key);
    let weather = ws.get_weather(&location)?;
    println!(
        "{} {}",
        weather.summary,
        if args.fahrenheit {
            format!("{:.1}ºF", weather.temperature.as_fahrenheit())
        } else {
            format!("{:.1}ºC", weather.temperature.as_celsius())
        }
    );
    Ok(())
}
