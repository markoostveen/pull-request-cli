mod request;
mod commands;

use std::env;
use crate::{request::account::get_account_identity, commands::review::print_pull_requests};

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    if args.len() == 1 || args[1].to_lowercase() == "--help"{
        commands::help::print_help();
        return;
    }
    
    if !commands::init::check_first_run() || args[1].to_lowercase() == "init"{
        commands::init::init_cli_tool();
        return;
    }

    if args[1].to_lowercase() == "review" {
        print_pull_requests(&args[2]);
        return;
    }

    println!("logged in user: '{}'", get_account_identity().login);
}