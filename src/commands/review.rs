use crate::request::repo::get_pull_requests;

pub fn print_pull_requests(owner: &String, project_name: &String){
    let request_list = get_pull_requests(owner, project_name);

    println!("Available pull-requests");
    for pull_request in request_list {
        println!("'{}': {}", pull_request.number, pull_request.title);
    }
}