#[allow(unused_imports)]
use std::io::{self, Write};

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
            _ => {
                print!("{}: command not found\n", command.trim());
            }
        }
    }
}
