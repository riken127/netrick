use colored::*;

pub fn success(message: &str) {
    println!("{}", message.green());
}

pub fn error(message: &str) {
    println!("{}", message.red());
}

pub fn info(message: &str) {
    println!("{}", message.blue());
}
