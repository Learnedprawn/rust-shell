//You give a line to the parser and it returns parsed input_vec outside
use std::mem::take;

pub fn parse_line(line: String) -> Result<(Vec<String>, Option<String>), ()> {
    let mut current_buffer: String = String::new();
    let mut input_vec: Vec<String> = vec![];
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut backslashed = false;
    let mut redirected = false;
    let mut redirection: Option<String> = None;
    let special = vec!['"', '\\'];
    for character in line.chars() {
        if backslashed {
            if in_double_quotes {
                if special.contains(&character) {
                    current_buffer.push(character);
                    backslashed = false;
                    continue;
                }
                current_buffer.push('\\');
                current_buffer.push(character);
                backslashed = false;
                continue;
            }
            current_buffer.push(character);
            backslashed = false;
            continue;
        }
        if character == '\\' {
            if in_single_quotes {
                current_buffer.push(character);
                continue;
            }
            if in_double_quotes {
                backslashed = true;
                continue;
            }
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
        if character == '>' {
            if !current_buffer.is_empty() {
                input_vec.push(take(&mut current_buffer));
            }
            redirected = true;
            continue;
        }
        if redirected {
            // println!(
            //     "Character is {}, current_buffer_status: {}",
            //     character,
            //     current_buffer.is_empty()
            // );
            if character == ' ' && !current_buffer.is_empty() {
                println!("is this happening");
                redirection = Some(take(&mut current_buffer));
                redirected = false;
            }
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
    if redirected {
        redirection = Some(current_buffer);
    } else {
        input_vec.push(current_buffer);
    }

    Ok((input_vec, redirection))
}
