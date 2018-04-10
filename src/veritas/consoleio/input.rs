use colored::*;
use veritas::consoleio::output::*;
use std::io::{Write, stdin, stdout};

pub fn get_console_input(query: &str, field: &str, optional: bool) -> Option<String> {
    print!("{}", &query.yellow().bold());
    stdout().flush().unwrap();
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
    input_str = input_str.trim().to_string();
    if !optional {
        while input_str.chars().count() == 0 {
            print_to_console(Status::Err, &format!("{} is invalid, please try again ", field));
            print!("\n{}", &query.yellow().bold());
            stdout().flush().unwrap();
            stdin().read_line(&mut input_str).ok().expect(&format!("{}", "Couldn't read line".red().bold()));
        }
        Some(input_str)
    } else {
        if input_str.chars().count() == 0 {
            None
        } else {
            Some(input_str)
        }

    }
}