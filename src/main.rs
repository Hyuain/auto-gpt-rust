use crate::helpers::command_line::get_user_response;

mod ai_functions;
mod apis;
mod helpers;
mod models;

fn main() {
    println!("Hello, world!");
    let user_request = get_user_response("What are we building today?");
    dbg!(user_request);
}
