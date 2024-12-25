#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::PathBuf, process::Command, sync::LazyLock};

static BUILTIN: LazyLock<Vec<&str>> = LazyLock::new(|| {
    let mut c = vec!["echo", "exit", "type", "pwd", "cd"];
    c.sort_unstable();
    c
});

fn parse_input(input: &str) -> Option<Vec<&str>> {
    let input = input.trim();
    let (cmd, args) = input.split_once(" ").unwrap_or((input, ""));
    let mut result = vec![cmd];

    let mut args = args.trim();
    while !args.is_empty() {
        match args.chars().next().unwrap() {
            '\'' => {
                let (arg, r) = args[1..].split_once('\'')?;
                result.push(arg);
                args = r
            },
            ' ' => {
                args = args.trim_start()
            },
            _ => {
                let (arg, r) = args.split_once(' ').unwrap_or((args, ""));
                result.push(arg);
                args = r
            }
        }
    }

    Some(result)
}


fn type_builtin(args: Vec<&str>, path: String) {
    args.iter().for_each(|cmd| {
        if BUILTIN.binary_search(cmd).is_ok() {
            println!("{} is a shell builtin", cmd);
        } else {
            let split = &mut path.split(":");
            if let Some(path) = 
                split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
                    println!("{} is {}/{}", cmd, path, cmd);
                } else {
                    println!("{}: not found", cmd);
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

fn pwd() {
    let current_dir = std::env::current_dir().unwrap();
    let path_str = current_dir.display();
    println!("{}", path_str);
}

fn cd(path: &str) {
    match path.bytes().nth(0) {
        Some(pre) => {
            let target_path = match pre {
                b'~' => {
                    let home_path = std::env::var("HOME").expect("Failed to get HOME environment variable");
                    PathBuf::from(home_path)
                    .join(if path.len() >= 2 {
                        &path[2..]
                    } else {
                        &path[1..]
                    })
                },
                b'.' => {
                    std::env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("/"))
                        .join(path)
                },
                _ => PathBuf::from(path),
            };
            if let Err(_) = std::env::set_current_dir(target_path) {
                println!("cd: {}: No such file or directory", path);
            }
        },
        None => println!("cd: {}: No such file or directory", path)
    }   
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let path_env = std::env::var("PATH").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let commands = parse_input(&input).expect("command parse error");
        if let Some(cmd) = commands.first() {
            match *cmd {
                "exit" => {
                    if commands.get(1).map_or(false, |x| *x == "0") {
                        break;
                    } else {
                        todo!();
                    }
                },
                "echo" => println!("{}", commands[1..].join(" ")),
                "type" => type_builtin(commands[1..].to_vec(), path_env.clone()),
                "pwd" => pwd(),
                "cd" => {
                    let Some(path) = commands.get(1) else {
                        continue;
                    };
                    cd(*path);
                },
                _ => {
                    if let Some(path) = find_executable_file(cmd, path_env.clone()) {
                        Command::new(path)
                        .args(&commands[1..])
                        .status()
                        .expect("failed to execute process");
                    } else {
                        println!("{}: command not found", input.trim());
                    }
                } 
            }
        }
        input.clear();
    }
}
