use serde::de::DeserializeOwned;

pub fn api_request<T: DeserializeOwned>(method: reqwest::Method, url : &String) -> Result<T, ()> {
    let client = reqwest::blocking::Client::new();

    // Build the request with headers and parameters
    let mut request = client.request(method, url);
    request = request.header(String::from("User-Agent"), String::from("request"));

    // Send request
    let response = match request.send(){
        Err(_e) => {
            println!("Request produced an error, '{}'", &url);
            return Err(());
        },
        Ok(response) => response,
    };

    // Make sure request was successful
    if response.status() != 200 {
        println!("Received statuscode: '{}' on request {}", response.status(), url);
        return Err(());
    }

    //println!("{}", response.text().unwrap());
    let resp = response.json::<T>();
    if resp.is_err() {
        return Err(println!("Unable to deserialize json '{}'", resp.err().unwrap().to_string()));
    }

    return Ok(resp.unwrap());
}