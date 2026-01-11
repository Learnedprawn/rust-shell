mod parser;

use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultPromptSegment, EditCommand, Emacs,
    FileBackedHistory, HistoryItem, KeyModifiers, MenuBuilder, Prompt, PromptEditMode,
    PromptHistorySearch, Reedline, ReedlineEvent, SearchDirection, SearchQuery, Signal,
    default_emacs_keybindings,
};
// use rustyline::{Config, DefaultEditor, Editor, completion::FilenameCompleter};
// use rustyline_derive::{Completer, Helper};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{borrow::Cow, env, fs::OpenOptions, os::fd::AsRawFd, process::Command, str::FromStr};

use is_executable::IsExecutable;

use crate::parser::{Redirection, parse_line};

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

// #[derive(Completer)]
// struct MyHelper {
//     #[rustyline(Completer)]
//     completer: FilenameCompleter,
// }

fn main() {
    // let config = Config::builder()
    //     .completion_type(rustyline::CompletionType::List)
    //     .build();
    // let h = MyHelper {
    //     completer: FilenameCompleter::new(),
    // };
    // let mut rl: Editor<Helper, _> = Editor::with_config(config).expect("rustyline init error");
    // let commands = vec![
    //     "test".into(),
    //     "hello world".into(),
    //     "hello world reedline".into(),
    //     "this is the reedline crate".into(),
    // ];
    let commands = vec!["echo".into(), "exit".into()];
    let history = Box::new(FileBackedHistory::new(100).expect("Error with history"));
    let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        reedline::KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".into()),
            ReedlineEvent::MenuNext,
        ]), // reedline::ReedlineEvent::UntilFound(vec![
            //     ReedlineEvent::Menu("completion_menu".to_string()),
            //     ReedlineEvent::MenuNext,
            // ]),
    );
    let edit_mode = Box::new(Emacs::new(keybindings));
    let mut rl = Reedline::create()
        .with_history(history)
        .with_completer(completer)
        .with_quick_completions(true)
        .with_menu(reedline::ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode);
    // let prompt = DefaultPrompt::default();
    impl Prompt for MyPrompt {
        fn render_prompt_left(&self) -> Cow<'_, str> {
            Cow::Borrowed("")
        }

        fn render_prompt_right(&self) -> Cow<'_, str> {
            Cow::Borrowed("")
        }

        fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<'_, str> {
            Cow::Borrowed("")
        }

        fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
            Cow::Borrowed("")
        }

        fn render_prompt_history_search_indicator(
            &self,
            _history_search: PromptHistorySearch,
        ) -> Cow<'_, str> {
            Cow::Borrowed("")
        }
    }
    struct MyPrompt {
        pub left_prompt: String,
        pub right_prompt: String,
    }

    // impl Default for MyPrompt {
    //     fn default() -> Self {
    //         MyPrompt {
    //             left_prompt: "",
    //             right_prompt: "",
    //         }
    //     }
    // }
    // let prompt = DefaultPrompt::new(DefaultPromptSegment::Empty, DefaultPromptSegment::Empty);
    let prompt = MyPrompt {
        left_prompt: "".to_string(),
        right_prompt: "".to_string(),
    };
    // rl.set_helper(Some(h));
    loop {
        rl.run_edit_commands(&[EditCommand::InsertString("$ ".to_string())]);
        let input = rl.read_line(&prompt);
        if let Ok(Signal::Success(input)) = input {
            // rl.add_history_entry(input.clone())
            //     .expect("Error in adding history");
            let buffer = input[2..].to_string();
            let (input_vec, redirection, err_redirection) = parse_line(buffer)
                .map_err(|e| {
                    eprintln!("Quoting error: {:?}", e);
                })
                .expect("parse_line failed");
            let saved_stdout;
            let saved_stderr;
            unsafe {
                saved_stdout = libc::dup(1);
                saved_stderr = libc::dup(2);
            }
            match redirection {
                Some(action) => match action {
                    Redirection::Redirect(path) => {
                        let file = std::fs::File::create(path).unwrap();
                        let file_fd = file.as_raw_fd();
                        unsafe {
                            libc::dup2(file_fd, 1);
                        }
                    }
                    Redirection::Append(path) => {
                        let file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)
                            .expect("Append File Opening error");
                        let file_fd = file.as_raw_fd();
                        unsafe {
                            libc::dup2(file_fd, 1);
                        }
                    }
                    _ => {}
                },
                None => {}
            };
            match err_redirection {
                Some(action) => match action {
                    Redirection::RedirectErr(path) => {
                        let file = std::fs::File::create(path).unwrap();
                        let file_fd = file.as_raw_fd();
                        unsafe {
                            libc::dup2(file_fd, 2);
                        }
                    }
                    Redirection::AppendErr(path) => {
                        let file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)
                            .expect("Append File Opening error");
                        let file_fd = file.as_raw_fd();
                        unsafe {
                            libc::dup2(file_fd, 2);
                        }
                    }
                    _ => {}
                },
                None => {}
            };

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
                    let history_list: Vec<HistoryItem> = rl
                        .history()
                        .search(SearchQuery::everything(SearchDirection::Forward, None))
                        .expect("History query failed");
                    match number {
                        Some(num) => {
                            let numeric_num: usize = num.parse().expect("parsing history failed");
                            for (i, exp) in history_list.iter().enumerate() {
                                if i + 1 > history_list.len() - numeric_num {
                                    println!("{} {}", i + 1, exp.command_line);
                                }
                            }
                        }
                        None => {
                            for (i, exp) in history_list.iter().enumerate() {
                                println!("{} {}", i + 1, exp.command_line);
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
                libc::dup2(saved_stderr, 2);
                libc::close(saved_stderr);
            }
        }
    }
}
