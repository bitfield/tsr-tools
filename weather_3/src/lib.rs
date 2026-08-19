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

#[expect(clippy::partial_pub_fields, reason = "api_key is private")]
pub struct Weatherstack {
    api_key: String,
    pub base_url: String,
}

impl Weatherstack {
    /// Fetches weather data from the Weatherstack API for the given location.
    ///
    /// # Errors
    ///
    /// Returns any errors making the request, from the server response, or from
    /// deserializing the JSON data.
    pub fn get_weather(&self, location: &str) -> Result<Weather> {
        let resp = request(&self.base_url, location, &self.api_key).send()?;
        let weather = deserialize(&resp.text()?)?;
        Ok(weather)
    }

    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            base_url: "https://api.weatherstack.com/current".into(),
        }
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

fn request(base_url: &str, location: &str, api_key: &str) -> RequestBuilder {
    Client::new()
        .get(base_url)
        .query(&[("query", location), ("access_key", api_key)])
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests")]
mod tests {
    use std::fs;

    use super::*;

    use http::StatusCode;
    use httpmock::{Method, MockServer};
    use url::Host::Domain;

    #[test]
    fn request_builds_correct_request() {
        let builder = request(
            "https://example.com/current",
            "London,UK",
            "dummy API key",
        );
        let req = builder.build().unwrap();
        assert_eq!(req.method(), "GET", "wrong method");
        let url = req.url();
        assert_eq!(url.host(), Some(Domain("example.com")), "wrong host");
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

    #[test]
    #[expect(clippy::string_add, reason = "clarity")]
    fn get_weather_fn_makes_correct_api_call() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(Method::GET)
                .path("/current")
                .query_param("query", "London,UK")
                .query_param("access_key", "dummy api key");
            then.status(StatusCode::OK)
                .header("content-type", "application/json")
                .body_from_file("tests/data/ws.json");
        });
        let mut ws = Weatherstack::new("dummy api key");
        ws.base_url = server.base_url() + "/current";
        let weather = ws.get_weather("London,UK");
        mock.assert();
        assert_eq!(
            weather.unwrap(),
            Weather {
                summary: "Sunny".into(),
                temperature: 11.2,
            },
            "wrong weather"
        );
    }
}
