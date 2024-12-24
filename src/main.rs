#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};

// split command with whitespace
fn tokenize(s: &str) -> Vec<&str> {
    s.split(" ").collect()
}

fn type_builtin(args: Vec<&str>, path: String) {
    args.iter().for_each(|cmd| {
        match *cmd {
            "echo" | "exit" | "type" => println!("{} is a shell builtin", cmd),
            _ => {
                let split = &mut path.split(":");
                if let Some(path) = 
                    split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
                        println!("{} is {}/{}", cmd, path, cmd);
                    } else {
                        println!("{}: not found", cmd);
                    }
            }
        }
    });
}

fn find_executable_file(file_name: &str , path: String) -> Option<String> {
    let split = &mut path.split(":");
    if let Some(path) = 
        split.find(|path| std::fs::metadata(format!("{}/{}", path, file_name)).is_ok()) {
            return Some(format!("{}/{}", path, file_name));
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let path_env = std::env::var("PATH").unwrap();
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
            ["type", ..] => type_builtin(tokens[1..].to_vec(), path_env.clone()),
            _ => {
                if let Some(path) = find_executable_file(tokens[0], path_env.clone()) {
                    Command::new(path)
                    .args(&tokens[1..])
                    .status()
                    .expect("failed to execute process");
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
        input.clear();
    }
}
