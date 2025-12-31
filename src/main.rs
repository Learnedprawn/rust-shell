mod parser;

use rustyline::DefaultEditor;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    fs::File,
    os::fd::AsRawFd,
    process::{Command, Stdio},
    str::FromStr,
};

use is_executable::IsExecutable;

use crate::parser::parse_line;

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
                // .stdout(out)
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

        rl.add_history_entry(input.clone())
            .expect("Error in adding history");
        let (input_vec, redirection) = parse_line(input)
            .map_err(|e| {
                eprintln!("Quoting error: {:?}", e);
            })
            .unwrap();
        let saved_stdout;
        unsafe {
            saved_stdout = libc::dup(1);
        }
        match redirection {
            Some(path) => {
                let file = std::fs::File::create(path).unwrap();
                let file_fd = file.as_raw_fd();
                unsafe {
                    libc::dup2(file_fd, 1);
                }
            }
            None => {}
        };

        // let out: Stdio;
        // out = match redirection {
        //     Some(path) => std::fs::File::create(path).unwrap().into(),
        //     None => Stdio::inherit(),
        // };

        // if in_single_quotes {
        //     panic!("Improper quotation use");
        // }

        let command = input_vec[0].as_str();

        match command {
            "exit" => break,
            "echo" => {
                // writeln!(&mut out, "{}", input_vec[1..].join(" ")).expect("echo write failed");
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
        unsafe {
            libc::dup2(saved_stdout, 1);
            libc::close(saved_stdout);
        }
    }
}
