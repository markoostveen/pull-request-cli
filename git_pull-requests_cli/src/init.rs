use std::path::Path;
use std::fs::File;
use std::io::Write;

static CONFIGFILENAME: &'static str = "git_pull-requests_cli.conf";

pub fn check_first_run() -> bool{
    return Path::new(CONFIGFILENAME).exists();
}

pub fn init_cli_tool(){
    println!("
    Welcome to Git Pull-Request CLI, in order to initialize there are a few steps in order to connect to your github.
    Please visit https://github.com/settings/tokens and create a new token.

    Paste in the created token and press enter.
    ");

    
    let file_result = File::create(CONFIGFILENAME);
    if file_result.is_err(){
        println!("Failed to create configfile");
        return;
    }

    let mut file = file_result.unwrap();
    
    let mut access_token = String::new();
    std::io::stdin().read_line(&mut access_token).expect("Unable to read userinput");
    
    file.write_all(access_token.as_bytes()).expect("Unable to write accesstoken to config file");
}