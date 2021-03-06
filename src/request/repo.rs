pub mod pull_request_list;
pub mod pull_request;
pub mod comment;

use std::collections::HashMap;
use reqwest::header::HeaderValue;
use crate::commands::init::pa_token;

use super::api_request;

pub fn get_pull_requests(owner: &String, project_name: &String) -> pull_request_list::PullRequestList {
    let url = format!("https://api.github.com/repos/{}/{}/pulls", owner, project_name);
    let token = String::from("token ") + &pa_token::read();

    let response =  api_request::<pull_request_list::PullRequestList>(reqwest::Method::GET, &url,
        HashMap::from(
            [
                (reqwest::header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
            ]
        ),
        None
    );

    match response{
        Ok(e) => return e,
        Err(_e) => return pull_request_list::PullRequestList::default()
    };
}

pub fn get_pull_request(owner: &String, project_name: &String, id: i32) -> pull_request::PullRequest {
    let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", owner, project_name, id);
    let token = String::from("token ") + &pa_token::read();

    let response =  api_request::<pull_request::PullRequest>(reqwest::Method::GET, &url,
        HashMap::from(
            [
                (reqwest::header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
            ]
        ),
        None
    );

    match response{
        Ok(e) => return e,
        Err(_e) => return pull_request::PullRequest::default()
    };
}

pub fn get_pull_request_comments(owner: &String, project_name: &String, id: i32) -> comment::PullRequestComment {
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", owner, project_name, id);
    let token = String::from("token ") + &pa_token::read();

    let response =  api_request::<comment::PullRequestComment>(reqwest::Method::GET, &url,
        HashMap::from(
            [
                (reqwest::header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
            ]
        ),
        None
    );

    match response{
        Ok(e) => return e,
        Err(_e) => return comment::PullRequestComment::default()
    };
}

pub fn add_pull_request_comment(owner: &String, project_name: &String, id: i32, comment: &String) -> pull_request::PullRequest {
    let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", owner, project_name, id);
    let token = String::from("token ") + &pa_token::read();

    let response =  api_request::<pull_request::PullRequest>(reqwest::Method::POST, &url,
        HashMap::from(
            [
                (reqwest::header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap())
            ]
        ),
        Some(format!("{{\"body\":\"{}\"}}", comment))
    );

    match response{
        Ok(e) => return e,
        Err(_e) => return pull_request::PullRequest::default()
    };
}