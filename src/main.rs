use rustyline::DefaultEditor;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::mem::take;
use std::{env, process::Command, str::FromStr};

use is_executable::IsExecutable;

enum CommandType {
    Builtin,
    File(String),
    NotFound,
}

impl CommandType {
    pub fn from_str(s: String) -> Self {
        match s.as_str() {
            "exit" => CommandType::Builtin,
            "echo" => CommandType::Builtin,
            "type" => CommandType::Builtin,
            "pwd" => CommandType::Builtin,
            "cd" => CommandType::Builtin,
            "history" => CommandType::Builtin,
            _ => match find_file(s.as_str()) {
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

pub fn find_file_and_execute(input: Vec<String>) -> Option<String> {
    let path = env::var("PATH").expect("Path Parsing error");
    let path_iterator = path.split(":");
    let command = input[0].clone();
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
    let mut rl = DefaultEditor::new().expect("rustyline init error");
    loop {
        let input = rl.readline("$ ").expect("line reading failed");
        // print!("$ ");
        // io::stdout().flush().unwrap();

        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();

        rl.add_history_entry(input.clone())
            .expect("Error in adding history");
        let mut current_buffer: String = String::new();
        let mut input_vec: Vec<String> = vec![];
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;
        let mut backslashed = false;
        for character in input.chars() {
            // println!(
            //     "character: {}, in_double_quotes: {}, backslashed: {}",
            //     character, in_double_quotes, backslashed
            // );
            if backslashed {
                current_buffer.push(character);
                backslashed = false;
                continue;
            }
            if character == '\\' {
                if in_single_quotes {
                    current_buffer.push(character);
                    continue;
                }
                // if in_double_quotes {
                //     backslashed = true;
                //     continue;
                // }
                backslashed = true;
                continue;
            }
            if character == '\'' && !in_double_quotes {
                if in_single_quotes {
                    in_single_quotes = false;
                } else {
                    in_single_quotes = true;
                }
                continue;
            }
            if character == '"' && !in_single_quotes {
                if in_double_quotes {
                    in_double_quotes = false;
                } else {
                    in_double_quotes = true;
                }
                continue;
            }
            if !in_single_quotes && !in_double_quotes {
                if character == ' ' && current_buffer.is_empty() {
                    continue;
                }
                if character == ' ' && !current_buffer.is_empty() {
                    input_vec.push(take(&mut current_buffer));
                    continue;
                }
            }
            current_buffer.push(character);
        }
        input_vec.push(current_buffer);
        if in_single_quotes {
            panic!("Improper quotation use");
        }

        let command = input_vec[0].as_str();

        // let literals: String = String::new();

        match command {
            "exit" => break,
            "echo" => {
                println!("{}", input_vec[1..].join(" "));
            }
            "type" => {
                let type_command = input_vec[1].clone();
                match CommandType::from_str(type_command.clone()) {
                    CommandType::Builtin => {
                        println!("{} is a shell builtin", type_command.clone())
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
                let number = input_vec.get(1);
                let history_list: Vec<&String> = rl.history().iter().collect();
                match number {
                    Some(num) => {
                        let numeric_num: usize = num.parse().expect("parsing history failed");
                        for (i, exp) in history_list.iter().enumerate() {
                            if i + 1 > history_list.len() - numeric_num {
                                println!("{} {}", i + 1, exp);
                            }
                        }
                    }
                    None => {
                        for (i, exp) in history_list.iter().enumerate() {
                            println!("{} {}", i + 1, exp);
                        }
                    }
                }
            }
            _ => match find_file_and_execute(input_vec.clone()) {
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
