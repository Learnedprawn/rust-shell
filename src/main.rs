#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process::Command};

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
    for path in path_iterator {
        let full_path = format!("{}/{}", path, command);
        if std::path::Path::new(&full_path).is_executable() {
            let output = Command::new("/bin/sh")
                .arg("-c")
                .args(input)
                .output()
                .expect("Found file execute error");
            println!("{:?}", output);
            return Some(String::from_utf8_lossy(&output.stdout).to_string());
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_iterator = input.trim().split(" ");

        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        println!("{:?}", input_vec);

        let command: &str = input_vec[0];

        match command {
            "exit" => break,
            "echo" => {
                for word in input_iterator {
                    print!("{} ", word);
                }
                print!("\n");
            }
            "type" => {
                let type_command = input_iterator.next().expect("type command error");
                match CommandType::from_str(type_command) {
                    CommandType::Builtin => {
                        println!("{} is a shell builtin", type_command)
                    }
                    CommandType::File(result) => {
                        println!("Hello: {}", result)
                    }
                    _ => {
                        println!("{} not found", type_command)
                    }
                }
            }
            _ => match find_file_and_execute(input_vec) {
                Some(result) => {
                    println!("{}", result)
                }
                None => {
                    print!("{}: command not found\n", command.trim());
                }
            },
        }
    }
}
