use std::env;

fn main() {
    let api_key = env::var("WEATHERSTACK_API_KEY").unwrap();
    let resp = reqwest::blocking::Client::new()
        .get("https://api.weatherstack.com/current")
        .query(&[("query", "London,UK"), ("access_key", &api_key)])
        .send()
        .unwrap();
    println!("{}", resp.text().unwrap());
}