pub mod account_info;

use std::collections::HashMap;

use reqwest::header::HeaderValue;

use crate::commands::init::pa_token;
use super::api_request;

pub fn get_account_identity() -> account_info::AccountInfo{
    let token = String::from("token ") + &pa_token::read();

    let url = String::from("https://api.github.com/user");
    let response =  api_request::<account_info::AccountInfo>(reqwest::Method::GET, &url,
        HashMap::from(
            [
                (reqwest::header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
            ]
        )
    );

    match response{
        Ok(e) => return e,
        Err(_e) => return account_info::AccountInfo::default()
    };

    //https://docs.github.com/en/rest/overview/resources-in-the-rest-api#authentication}
}