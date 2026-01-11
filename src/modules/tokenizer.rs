pub fn tokenizer(cmd: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    // Use a peekable iterator to look at the next character without moving past it
    let mut chars = cmd.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // 1. Handle Whitespace: Ends the current word and skips extra spaces
            ' ' | '\t' | '\n' | '\r' => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                // Skip all subsequent whitespace
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }

            // 2. Handle Quotes: Everything inside stays as one single token
            '"' | '\'' => {
                let quote_char = ch;
                while let Some(inner_ch) = chars.next() {
                    if inner_ch == quote_char {
                        break; // Closing quote found
                    } else if inner_ch == '\\' {
                        // Handle escape sequences inside quotes (e.g., \" or \n)
                        if let Some(escaped) = chars.next() {
                            match escaped {
                                'n' => current_token.push('\n'),
                                't' => current_token.push('\t'),
                                '\\' => current_token.push('\\'),
                                c if c == quote_char => current_token.push(quote_char),
                                _ => {
                                    current_token.push('\\');
                                    current_token.push(escaped);
                                }
                            }
                        }
                    } else {
                        current_token.push(inner_ch);
                    }
                }
            }

            // 3. Handle Backslashes: Treats the next character literally
            '\\' => {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        '\\' => current_token.push('\\'),
                        _ => current_token.push(escaped),
                    }
                }
            }

            // 4. Handle Normal Characters: Build the word character by character
            _ => {
                current_token.push(ch);
            }
        }
    }

    // Push the very last word if there wasn't a trailing space
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // Return an error if the user just hit Enter without typing anything
    if tokens.is_empty() {
        Err("Write Your Command".to_string())
    } else {
        Ok(tokens)
    }
}