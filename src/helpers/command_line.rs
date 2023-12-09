use std::io::{stdin, stdout};
use crossterm::ExecutableCommand;
use crossterm::style::{Color, ResetColor, SetForegroundColor};

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!();
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespaces and return
    user_response.trim().to_string()
}
