use anyhow::Result;
use clap::{CommandFactory, Parser};

use std::env;

use weather::get_weather;
use weather_2 as weather;

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
    if env::args().count() < 2 {
        Args::command().print_long_help()?;
        return Ok(());
    }
    let args = Args::parse();
    let location = args.location.join(" ");
    let weather = get_weather(&location, &args.api_key)?;
    println!("{weather}");
    Ok(())
}
