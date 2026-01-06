//You give a line to the parser and it returns parsed input_vec outside
use std::mem::take;

pub fn parse_line(line: String) -> Result<(Vec<String>, Option<String>, Option<String>), ()> {
    let mut current_buffer: String = String::new();
    let mut input_vec: Vec<String> = vec![];
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut backslashed = false;
    let mut redirected = false;
    let mut err_redirected = false;
    let mut redirection: Option<String> = None;
    let mut err_redirection: Option<String> = None;
    let special = vec!['"', '\\'];
    let mut line_iter = line.chars().peekable();
    // for character in line_iter.by_ref() {
    loop {
        match line_iter.next() {
            Some(character) => {
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
                if character == '2' && matches!(line_iter.peek(), Some('>')) {
                    println!("This is called");
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    err_redirected = true;
                    line_iter.next();
                    continue;
                }
                if character == '>' || (character == '1' && matches!(line_iter.peek(), Some('>'))) {
                    println!("Also This is called");
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    redirected = true;
                    continue;
                }
                if err_redirected {
                    println!("Then This is called");
                    if character == ' ' && !current_buffer.is_empty() {
                        println!("Inside");
                        err_redirection = Some(take(&mut current_buffer));
                        err_redirected = false;
                    }
                }
                if redirected {
                    if character == ' ' && !current_buffer.is_empty() {
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
            None => break,
        }
        // }
    }
    if redirected {
        redirection = Some(current_buffer);
    } else if err_redirected {
        err_redirection = Some(current_buffer);
    } else {
        input_vec.push(current_buffer);
    }

    Ok((input_vec, redirection, err_redirection))
}
