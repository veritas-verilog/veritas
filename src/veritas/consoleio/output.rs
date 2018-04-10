use colored::*;

#[derive(Debug)]
pub enum Status {
    Err,
    Info,
    Warn,
    Special,
}

pub fn print_to_console(status: Status, msg: &str) {
    let msg_str = match status {
        // REALLY REALLY DIRTY
        Status::Err => format!("{}", format!("ERROR: {}", msg).red().bold()),
        Status::Info => format!("{}: {}", "INFO".bold(), msg),
        Status::Warn => format!("{}: {}","WARNING:".yellow().bold(), msg),
        Status::Special => format!("{}", msg.green().bold())
    };
    println!("{}", msg_str);
} 