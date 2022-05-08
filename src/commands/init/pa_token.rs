use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};

static CONFIGFILENAME: &'static str = "git_pull-requests_cli.conf";

fn is_escaped(c: char)-> bool {
    return c == '\r' || c == '\n';
}

fn token_valid(access_token: &String) -> bool{
    return !access_token.starts_with(&String::from("ghp_"));
}

pub fn exists() -> bool{
    return Path::new(CONFIGFILENAME).exists();
}

pub fn create(access_token: String){

    if token_valid(&access_token) {
        println!("Error: Invalid input, personal access tokens usually start with ghp_");
        println!("command failed");
        return;
    }

    // Create file to store accesstoken into, will trunc in case file exists
    let file_result = File::create(CONFIGFILENAME);
    if file_result.is_err(){
        println!("Error: Failed to create configfile");
        return;
    }

    let mut file = file_result.unwrap();
    file.write_all(access_token.as_bytes()).expect("Error: Unable to write accesstoken to config file");
}

pub fn read() -> String{
    let mut file = File::open(CONFIGFILENAME).expect("Error: Personal Access Token config not found");

    let mut token = String::new();
    file.read_to_string(&mut token).expect("Unable to read file");

    if token_valid(&token) {
        println!("Error: Invalid token read");
        return String::from("");
    }

    return token.replace(|c: char| !c.is_ascii() || is_escaped(c), "");
}