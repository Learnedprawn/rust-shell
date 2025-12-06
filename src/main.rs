use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

enum CommandType {
    Builtin,
    File,
    NotFound,
}

impl CommandType {
    pub fn from_str(s: &str, filepath: &mut &String) -> Self {
        match s {
            "exit" => CommandType::Builtin,
            "echo" => CommandType::Builtin,
            "type" => CommandType::Builtin,
            command if find_file(s, filepath) => CommandType::File,
            _ => CommandType::NotFound,
        }
    }
}

pub fn find_file(s: &str, filepath: &mut &String) -> bool {
    let path = env::var("PATH").expect("Path Parsing error");
    let path_iterator = path.split(":");
    for path in path_iterator {
        let full_path = format!("{}/{}", path, s);
        if std::path::Path::new(&full_path).exists() {
            println!("{} is {}", s, full_path);
            return true;
        }
    }
    false
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut input_iterator = input.trim().split(" ");

        let command: &str = input_iterator.next().expect("Command parse error");

        // println!("Command: {:?}", command);
        // let path = env::var("PATH");
        // println!("PATH: {:?}", path);

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
                let filepath = String::new();
                match CommandType::from_str(type_command, &mut &filepath) {
                    CommandType::Builtin => {
                        println!("{} is a shell builtin", type_command)
                    }
                    CommandType::File => {
                        // output is being printed in the find_file function. might have to change
                        // this later
                    }
                    _ => {
                        println!("{} not found", type_command)
                    }
                }
            }
            _ => {
                print!("{}: command not found\n", command.trim());
            }
        }
    }
}
