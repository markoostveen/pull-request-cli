use crate::request::repo::{get_pull_requests, get_pull_request, pull_request, get_pull_request_comments};

pub fn print_pull_requests(owner: &String, project_name: &String){
    let request_list = get_pull_requests(owner, project_name);

    println!("Available pull-requests");
    for pull_request in request_list {
        println!("'{}': {}", pull_request.number, pull_request.title);
    }
}

pub fn print_pull_request(owner: &String, project_name: &String, id: i32){
    let pull_request = get_pull_request(owner, project_name, id);

    if pull_request == pull_request::PullRequest::default() {
        println!("Unable to find pull-request with id {}", &id);
        return;
    }


    let comments = get_pull_request_comments(owner, project_name, id);

    println!("
        Pull-request '{}'
        assignee '{}'
        summery '{}'
        comments: {}
        ",
        pull_request.title,
        pull_request.assignees
            .into_iter()
            .map(|x|
                serde_json::from_str::<pull_request::Assignee>(
                    x.as_str()
                    .unwrap_or(""))
                    .unwrap_or(pull_request::Assignee::default()
                ).login)
                .fold(
                    String::from(""),
                     |output, x|
                     format!("{},{}", &output, &x)
                    ), 
        pull_request.body,
        comments.into_iter().fold(String::from(""),
        |output, x|
        format!("\n{} {} said: {},", &output, x.user.login, x.body)
    )
    );
}