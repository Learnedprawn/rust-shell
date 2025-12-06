#[allow(unused_imports)]
use std::io::{self, Write};

enum CommandType {
    Builtin,
    File,
    NotFound,
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "exit" => CommandType::Builtin,
            "echo" => CommandType::Builtin,
            "type" => CommandType::Builtin,
            _ => CommandType::NotFound,
        }
    }
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
