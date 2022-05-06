use std::{collections::HashMap};

pub fn api_request(url : String) -> Result<String, ()> {
    let client = reqwest::blocking::Client::new();
    client.post(url);
    let response = match reqwest::blocking::get(url.clone()){
        Err(_e) => {
            println!("Request had an error");
            return Err(());
        },
        Ok(response) => response,
    };

    if response.status() != 200 {
        println!("Received statuscode: '{}' on request {}", response.status(), url);
        return Err(());
    }

    let resp = response.json::<HashMap<String,String>>();

    if resp.is_err() {
        println!("Unable to deserialize json");
        return Err(());
    }

    return Ok(String::from("OK"));
}