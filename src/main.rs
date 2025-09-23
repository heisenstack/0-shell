#[allow(unused_imports)]
use std::io::{self, Write};
mod modules;
use modules::prossess::*;
use modules::tokenizer::*;

#[derive(PartialEq)]
enum QuoteState {
    None,
    Single,
    Double,
}

fn main() {
    loop {
        let mut full_input = String::new();
        let mut quote_state = QuoteState::None;
        let mut first_line = true;

        loop {
            if first_line {
                print!("$ ");
            } else {
                print!("> ");
            }

            if let Err(e) = io::stdout().flush() {
                eprintln!("Failed to flush stdout: {}", e);
                return;
            }

            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => {
                    if !first_line {
                        eprintln!();
                        break;
                    } else {
                        println!();
                        return;
                    }
                }
                Ok(_) => {
                    quote_state = update_quote_state(&line, quote_state);

                    let line_continuation = is_line_continuation(&line);

                    if line_continuation {
                        let mut trimmed_line = line.trim_end().to_string();
                        trimmed_line.pop();
                        full_input.push_str(&trimmed_line);
                        first_line = false;
                        continue;
                    }

                    full_input.push_str(&line);

                    if quote_state == QuoteState::None {
                        break;
                    }

                    first_line = false;
                }
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    return;
                }
            }
        }

        let input = full_input.trim();

        if input.is_empty() {
            continue;
        }

        let tokens = match tokenizer(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        match prossess(tokens) {
            Ok(v) => {
                if v.as_str() == "exit" {
                    print!("exit\n");
                    break;
                } else {
                    print!("{}", v)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn is_line_continuation(line: &str) -> bool {
    let trimmed = line.trim_end();
    if !trimmed.ends_with('\\') {
        return false;
    }

    let mut backslash_count = 0;
    for ch in trimmed.chars().rev() {
        if ch == '\\' {
            backslash_count += 1;
        } else {
            break;
        }
    }

    backslash_count % 2 == 1
}

fn update_quote_state(line: &str, mut current_state: QuoteState) -> QuoteState {
    let mut is_esc = false;

    for ch in line.chars() {
        if is_esc {
            is_esc = false;
            continue;
        }

        match ch {
            '\\' => {
                if current_state != QuoteState::Single {
                    is_esc = true;
                }
            }
            '\'' => match current_state {
                QuoteState::Single => current_state = QuoteState::None,
                QuoteState::None => current_state = QuoteState::Single,
                QuoteState::Double => {}
            },
            '"' => match current_state {
                QuoteState::Double => current_state = QuoteState::None,
                QuoteState::None => current_state = QuoteState::Double,
                QuoteState::Single => {}
            },
            _ => {}
        }
    }

    current_state
}
