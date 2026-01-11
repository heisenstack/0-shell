#[allow(unused_imports)]
use std::io::{self, Write};
mod modules;
use modules::prossess::*;
use modules::tokenizer::*;

// Tracks if we are currently inside quotes to handle multi-line input
#[derive(PartialEq)]
enum QuoteState {
    None,
    Single,
    Double,
}

fn main() {
    // 1. The main shell loop: keeps the program running until you type 'exit'
    loop {
        let mut full_input = String::new();
        let mut quote_state = QuoteState::None;
        let mut first_line = true;

        // 2. The input gathering loop: handles multi-line input (quotes or backslashes)
        loop {
            // Show '$ ' for a new command, or '> ' if we are continuing a previous line
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
                Ok(0) => { // Handle End-of-File (Ctrl+D)
                    if !first_line {
                        eprintln!();
                        break;
                    } else {
                        println!();
                        return;
                    }
                }
                Ok(_) => {
                    // Update state to see if we opened or closed quotes
                    quote_state = update_quote_state(&line, quote_state);

                    // Check if the line ends in a backslash ( \ ) for continuation
                    let line_continuation = is_line_continuation(&line);

                    if line_continuation {
                        let mut trimmed_line = line.trim_end().to_string();
                        trimmed_line.pop(); // Remove the trailing backslash
                        full_input.push_str(&trimmed_line);
                        first_line = false;
                        continue; // Keep reading the next line
                    }

                    full_input.push_str(&line);

                    // If quotes are closed and no backslash continuation, command is complete
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

        // 3. Tokenize the input (break "ls -l" into ["ls", "-l"])
        let tokens = match tokenizer(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // 4. Process and execute the command
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

/// Detects if the line ends with an unescaped backslash, meaning the command continues
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

    // Only an odd number of backslashes at the end means "continue"
    backslash_count % 2 == 1
}

/// Scans the line to toggle between quote states
fn update_quote_state(line: &str, mut current_state: QuoteState) -> QuoteState {
    let mut is_esc = false;

    for ch in line.chars() {
        if is_esc {
            is_esc = false;
            continue;
        }

        match ch {
            '\\' => {
                // Backslashes don't escape inside single quotes
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