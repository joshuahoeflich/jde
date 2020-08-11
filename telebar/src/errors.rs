use colored::Colorize;

pub fn error_message(error: &str, suggestion: &str) {
    eprintln!("{}", error.bold().red());
    eprintln!("{}", suggestion.yellow());
}
