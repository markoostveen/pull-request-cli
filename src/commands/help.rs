

pub fn print_help() {
    println!("
    Use --help to view all supported commands for cli tool
    usage:
        git_pull-requests_cli --help
        git_pull-requests_cli init
        git_pull-requests_cli view <OWNER> <PROJECT_NAME>
        git_pull-requests_cli view <OWNER> <PROJECT_NAME> <PULL-REQUEST_ID>
    ");
}