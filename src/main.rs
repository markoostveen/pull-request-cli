mod request;
mod commands;

use std::env;
use crate::{request::{account::get_account_identity, repo::add_pull_request_comment}, commands::review::{print_pull_requests, print_pull_request}};

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

    if args[1].to_lowercase() == "view" {
        if args.len() == 4 {
            print_pull_requests(&args[2], &args[3]);
            return;
        }
        else if args.len() == 5{
            print_pull_request(&args[2], &args[3], args[4].parse().unwrap());
            return;
        }
    }

    if args[1].to_lowercase() == "comment" {
        add_pull_request_comment(&args[2], &args[3], args[4].parse().unwrap(), &args[5]);
        return;
    }

    println!("logged in user: '{}'", get_account_identity().login);
    commands::help::print_help();
}