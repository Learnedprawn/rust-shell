#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process::Command, str::FromStr};

use is_executable::IsExecutable;

enum CommandType {
    Builtin,
    File(String),
    NotFound,
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "exit" => CommandType::Builtin,
            "echo" => CommandType::Builtin,
            "type" => CommandType::Builtin,
            "pwd" => CommandType::Builtin,
            "cd" => CommandType::Builtin,
            "history" => CommandType::Builtin,
            _ => match find_file(s) {
                Some(result) => CommandType::File(result),
                None => CommandType::NotFound,
            },
        }
    }
}

pub fn find_file(s: &str) -> Option<String> {
    let path = env::var("PATH").expect("Path Parsing error");
    let path_iterator = path.split(":");
    for path in path_iterator {
        let full_path = format!("{}/{}", path, s);
        if std::path::Path::new(&full_path).is_executable() {
            // println!("{} is {}", s, full_path);
            let result = format!("{} is {}", s, full_path);
            return Some(result);
        }
    }
    None
}

pub fn find_file_and_execute(input: Vec<&str>) -> Option<String> {
    let path = env::var("PATH").expect("Path Parsing error");
    let path_iterator = path.split(":");
    let command = input[0];
    let args = &input[1..];
    for path in path_iterator {
        let full_path = format!("{}/{}", path, command);
        if std::path::Path::new(&full_path).is_executable() {
            // let mut handle = Command::new("/bin/sh")
            //     .arg("-c")
            //     .arg(command)
            //     .args(args)
            //     .spawn()
            //     .expect("Found file execute error");
            let mut handle = Command::new(command)
                .args(args)
                .spawn()
                .expect("handle failed");

            handle.wait().expect("handle failed");

            return Some(String::from_str("Done").unwrap());
        }
    }
    None
}

fn main() {
    let mut history_list: Vec<String> = vec![];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_vec: Vec<&str> = input.trim().split(" ").collect();

        let command: &str = input_vec[0];
        history_list.push(input[..input.len() - 1].to_string());

        match command {
            "exit" => break,
            "echo" => {
                for word in input_vec[1..].iter() {
                    print!("{} ", word);
                }
                print!("\n");
            }
            "type" => {
                let type_command = input_vec[1];
                match CommandType::from_str(type_command) {
                    CommandType::Builtin => {
                        println!("{} is a shell builtin", type_command)
                    }
                    CommandType::File(result) => {
                        println!("{}", result)
                    }
                    _ => {
                        println!("{} not found", type_command)
                    }
                }
            }
            "pwd" => {
                let pwd = env::current_dir().expect("pwd fetch error");
                println!("{}", pwd.to_str().expect("pwd string parsing failed"));
            }
            "cd" => {
                let mut path = input_vec[1].to_string();
                if input_vec[1].starts_with("~") {
                    let home = env::var("HOME").expect("Home ENV variable");
                    path = format!("{}{}", home, &input_vec[1][1..]);
                }
                if let Err(_e) = env::set_current_dir(&path) {
                    println!("cd: {}: No such file or directory", path)
                }
            }
            "history" => {
                for (i, exp) in history_list.iter().enumerate() {
                    println!("{} {}", i + 1, exp);
                }
            }
            _ => match find_file_and_execute(input_vec) {
                Some(_result) => {
                    // println!("{}", result)
                }
                None => {
                    print!("{}: command not found\n", command.trim());
                }
            },
        }
    }
}
