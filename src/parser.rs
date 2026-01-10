//You give a line to the parser and it returns parsed input_vec outside
use std::{mem::take, path::PathBuf};

pub enum Redirection {
    Redirect(PathBuf),
    RedirectErr(PathBuf),
    Append(PathBuf),
    AppendErr(PathBuf),
}

pub fn parse_line(
    line: String,
) -> Result<(Vec<String>, Option<Redirection>, Option<Redirection>), ()> {
    let mut current_buffer: String = String::new();
    let mut input_vec: Vec<String> = vec![];
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut backslashed = false;
    let mut redirected = false;
    let mut appended = false;
    let mut err_redirected = false;
    let mut redirection: Option<Redirection> = None;
    let mut err_redirection: Option<Redirection> = None;
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
                if character == '>' && matches!(line_iter.peek(), Some('>')) {
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    appended = true;
                    line_iter.next();
                    continue;
                }
                if character == '2' && matches!(line_iter.peek(), Some('>')) {
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    err_redirected = true;
                    line_iter.next();
                    continue;
                }
                if character == '>' {
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    redirected = true;
                    continue;
                }
                if character == '1' && matches!(line_iter.peek(), Some('>')) {
                    if !current_buffer.is_empty() {
                        input_vec.push(take(&mut current_buffer));
                    }
                    line_iter.next();
                    if matches!(line_iter.peek(), Some('>')) {
                        line_iter.next();
                        appended = true;
                        continue;
                    }
                    redirected = true;
                    continue;
                }
                if err_redirected {
                    if character == ' ' && !current_buffer.is_empty() {
                        err_redirection = Some(Redirection::RedirectErr(PathBuf::from(take(
                            &mut current_buffer,
                        ))));
                        err_redirected = false;
                    }
                }
                if redirected {
                    if character == ' ' && !current_buffer.is_empty() {
                        redirection = Some(Redirection::Redirect(PathBuf::from(take(
                            &mut current_buffer,
                        ))));
                        redirected = false;
                    }
                }
                if appended {
                    if character == ' ' && !current_buffer.is_empty() {
                        redirection = Some(Redirection::Append(PathBuf::from(take(
                            &mut current_buffer,
                        ))));
                        appended = false;
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
        redirection = Some(Redirection::Redirect(PathBuf::from(current_buffer)));
    } else if err_redirected {
        err_redirection = Some(Redirection::RedirectErr(PathBuf::from(current_buffer)));
    } else if appended {
        redirection = Some(Redirection::Append(PathBuf::from(current_buffer)));
    } else {
        input_vec.push(current_buffer);
    }

    Ok((input_vec, redirection, err_redirection))
}
