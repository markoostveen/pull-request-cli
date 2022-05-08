mod request;
mod commands;

use std::env;
use crate::{request::{account::get_account_identity, repo::add_pull_request_comment}, commands::review::{print_pull_requests, print_pull_request}};

use clap::{Arg, App, SubCommand, Command, Subcommand};

fn main() {

    let mut app = App::new("pull-request-cli")
        .about("tool to help view & update pull-requests on github projects")
        .subcommand(
            Command::new("init")
            .about("Initialize cli-tool and connect to github using personal access token")
        )
        .subcommand(
            Command::new("view")
            .about("View pull-requests of a github repo")
            .arg(
                Arg::new("owner")
                .required(true)
                .help("owner of the repo that needs to be looked up")
            )
            .arg(
                Arg::new("project_name")
                .required(true)
                .help("Name of project")
            )
            .arg(
                Arg::new("id")
                .help("Id of the pull-request as displayed in github")
            )
        )
        .subcommand(
            Command::new("comment")
            .about("Post a comment on an existing pull-request")
            .arg(
                Arg::new("owner")
                .required(true)
                .help("owner of the target repository")
            )
            .arg(
                Arg::new("project_name")
                .required(true)
                .help("name of the target project")
            )
            .arg(
                Arg::new("id")
                .required(true)
                .help("id of the pull-request as listed on github")
            )
            .arg(
                Arg::new("message")
                .required(true)
                .help("string to be posted as the message on the new comment")
            )
        )
        .subcommand(
            Command::new("Identity")
            .about("Print identity of logged in user")
        );

    // Initialize command
    let matches = app.clone().get_matches();
    match matches.subcommand_matches("init") {
        Some(_e) => { commands::init::init_cli_tool(); return; },
        None => {}
    };

    if !commands::init::check_first_run() {
        println!("Cli tool not initialized, please invoke init command");
        return
    }

    // view subcommand
    match matches.subcommand_matches("view") {
        Some(e) => {
            let owner = e.value_of("owner").unwrap().to_string();
            let project_name = e.value_of("project_name").unwrap().to_string();
            if e.is_present("id"){
                let id = e.value_of("id").unwrap().parse().unwrap();
                print_pull_request(&owner, &project_name, id);
            }
            else{
                print_pull_requests(&owner, &project_name);
            }
            return;
        },
        None => {}
    };

    // comment subcommand
    match matches.subcommand_matches("comment") {
        Some(e) => {
            let owner = e.value_of("owner").unwrap().to_string();
            let project_name = e.value_of("project_name").unwrap().to_string();
            let id = e.value_of("id").unwrap().parse().unwrap();
            let message = e.value_of("message").unwrap().to_string();
            add_pull_request_comment(&owner, &project_name, id, &message);
            return;
        },
        None => {}
    };

    // command identity
    match matches.subcommand_matches("identity") {
        Some(_e) => {
            println!("logged in user: '{}'", get_account_identity().login);
            return;
        },
        None => {}
    };

    app.print_long_help();

    return;
}