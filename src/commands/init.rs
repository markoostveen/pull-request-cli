pub mod pa_token;

pub fn check_first_run() -> bool{
    return pa_token::exists();
}

pub fn init_cli_tool(){
    println!("
    Welcome to Git Pull-Request CLI, in order to initialize there are a few steps in order to connect to your github.
    Please visit https://github.com/settings/tokens and create a new token.

    Paste in the created token and press enter.
    ");

    
    // Wait for userinput to paste access key, and write to file
    let mut access_token = String::new();
    std::io::stdin().read_line(&mut access_token).expect("Error: Unable to read userinput");

    pa_token::create(access_token);

    println!("\n CLI tool is now setup")
}