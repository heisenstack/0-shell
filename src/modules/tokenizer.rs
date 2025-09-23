pub fn tokenizer(cmd: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut chars = cmd.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }

            '"' | '\'' => {
                let quote_char = ch;
                while let Some(inner_ch) = chars.next() {
                    if inner_ch == quote_char {
                        break;
                    } else if inner_ch == '\\' {
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

            '\\' => {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        '\\' => current_token.push('\\'),
                        _ => current_token.push(escaped),
                    }
                }
            }

            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    if tokens.is_empty() {
        Err("Write Your Command".to_string())
    } else {
        Ok(tokens)
    }
}
