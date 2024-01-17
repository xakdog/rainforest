use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, ACCEPT};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    message: String,
    follow: Option<String>,
}

fn fetch_json(url: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = Client::new();
    let response = client.get(url).headers(headers).send()?;

    if response.status().is_success() {
        let parsed_response: ApiResponse = response.json()?;
        Ok(parsed_response)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed",
        )))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut api_response = ApiResponse {
        // I started solving this puzzle with CURL
        message: "Journey into the unknown".to_string(),
        follow: Some("https://www.letsrevolutionizetesting.com/challenge?id=756775492".to_string()),
    };

    while let Some(follow) = &api_response.follow {
        api_response = fetch_json(follow)?;
        println!("{:?}", api_response);
    }

    Ok(())
}
