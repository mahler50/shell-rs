#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn tokenize(s: &str) -> Vec<&str> {
    s.split(" ").collect()
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();
        let tokens = tokenize(command);
        match tokens[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            ["echo", ..] => println!("{}", tokens[1..].join(" ")),
            _ => {
                println!("{}: command not found", command);
            }
        }
        input.clear();
    }
}
