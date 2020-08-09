use colored::Colorize;

pub fn error_message(error: &str, suggestion: String) {
    eprintln!("{}", error.bold().red());
    eprintln!("{}", suggestion.yellow());
}
