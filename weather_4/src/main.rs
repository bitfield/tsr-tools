use anyhow::Result;
use clap::{CommandFactory as _, Parser};

use std::env;

use weather::Weatherstack;
use weather_4 as weather;

#[derive(Parser)]
/// Shows the current weather for a given location.
struct Args {
    /// Weatherstack API key.
    #[arg(short, long, env = "WEATHERSTACK_API_KEY", required = true)]
    api_key: String,
    /// Report temperatures in Fahrenheit.
    #[arg(short, long)]
    fahrenheit: bool,
    /// Example: "London,UK".
    #[arg(required = true)]
    location: Vec<String>,
}

fn main() -> Result<()> {
    if env::args().count() < 2 {
        Args::command().print_long_help()?;
        return Ok(());
    }
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
