pub mod account;
pub mod repo;

use std::collections::HashMap;

use reqwest::header::{HeaderName, HeaderValue};
use serde::de::DeserializeOwned;

pub fn api_request<T: DeserializeOwned>(method: reqwest::Method, url : &String, headers: HashMap<HeaderName,HeaderValue>, body: Option<String>) -> Result<T, ()> {
    let client = reqwest::blocking::Client::new();

    // Build the request with headers and parameters
    let mut request = client.request(method, url);
    request = request.header(reqwest::header::USER_AGENT, String::from("request"));

    for (key, value) in headers{
        request = request.header(key, value);
    }
    if body.is_some() {
        request = request.body(body.unwrap());
    }

    // Send request
    let response = match request.send(){
        Err(_e) => {
            return Err(println!("Request produced an error, '{}' with '{}'", &url, _e));
        },
        Ok(response) => response,
    };

    // Make sure request was successful
    if response.status() != 200 {
        return Err(println!("Received statuscode: '{}' on request {}", response.status(), url));
    }

    //println!("{}", response.text().unwrap());
    let resp_text = response.text().unwrap();
    let resp = serde_json::from_str::<T>(&resp_text);
    if resp.is_err() {
        return Err(println!("Unable to deserialize json '{}', {}\n{}", resp.err().unwrap().to_string(), url, resp_text));
    }

    return Ok(resp.unwrap());
}