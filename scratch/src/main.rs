use anyhow::Result;

use std::env;

use scratch as weather;
use weather::get_weather;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let location = args.join(" ");
    let api_key = env::var("WEATHERSTACK_API_KEY")?;
    let weather = get_weather(&location, &api_key)?;
    println!("{weather}");
    Ok(())
}
