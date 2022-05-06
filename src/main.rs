mod request;
mod help;
mod init;

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

    println!("{:#?}", request::api_request(String::from("https://api.github.com/users/defunkt")));
}