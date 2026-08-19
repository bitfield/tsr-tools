use anyhow::{Context as _, Result};

use std::fmt::{Display, Formatter};

use reqwest::blocking::{Client, RequestBuilder};
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Weather {
    summary: String,
    temperature: f64,
}

impl Display for Weather {
    #[expect(clippy::absolute_paths, reason = "not anyhow::Result")]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.1}ºC", self.summary, self.temperature)
    }
}

fn deserialize(json: &str) -> Result<Weather> {
    let val: Value = serde_json::from_str(json)?;
    let temperature = val
        .pointer("/current/temperature")
        .and_then(Value::as_f64)
        .with_context(|| format!("bad response: {val}"))?;
    let summary = val
        .pointer("/current/weather_descriptions/0")
        .and_then(Value::as_str)
        .with_context(|| format!("bad response: {val}"))?
        .to_owned();
    Ok(Weather {
        summary,
        temperature,
    })
}

/// Fetches weather data from the Weatherstack API for the given location.
///
/// # Errors
///
/// Returns any errors making the request, from the server response, or from
/// deserializing the JSON data.
pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
    let resp = request(location, api_key).send()?;
    let weather = deserialize(&resp.text()?)?;
    Ok(weather)
}

fn request(location: &str, api_key: &str) -> RequestBuilder {
    Client::new()
        .get("https://api.weatherstack.com/current")
        .query(&[("query", location), ("access_key", api_key)])
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests")]
mod tests {
    use std::fs;

    use super::*;

    use url::Host::Domain;

    #[test]
    fn request_builds_correct_request() {
        let builder = request("London,UK", "dummy API key");
        let req = builder.build().unwrap();
        assert_eq!(req.method(), "GET", "wrong method");
        let url = req.url();
        assert_eq!(
            url.host(),
            Some(Domain("api.weatherstack.com")),
            "wrong host"
        );
        assert_eq!(url.path(), "/current", "wrong path");
        let params: Vec<(_, _)> = url.query_pairs().collect();
        assert_eq!(
            params,
            vec![
                ("query".into(), "London,UK".into()),
                ("access_key".into(), "dummy API key".into())
            ],
            "wrong params"
        );
    }

    #[test]
    fn deserialize_extracts_correct_weather_from_json() {
        let json = fs::read_to_string("tests/data/ws.json").unwrap();
        let weather = deserialize(&json).unwrap();
        assert_eq!(
            weather,
            Weather {
                summary: "Sunny".into(),
                temperature: 11.2,
            },
            "wrong weather"
        );
    }
}
