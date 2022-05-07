mod request;
mod help;
mod init;
mod request_types;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 || args[1].to_lowercase() == "--help"{
        help::print_help();
        return;
    }

    if !init::check_first_run() || args[1].to_lowercase() == "init"{
        init::init_cli_tool();
        return;
    }

    let url = String::from("https://api.github.com/users/octocat");
    println!("{:#?}", request::api_request::<request_types::Test::ABC>(reqwest::Method::GET, &url));
}