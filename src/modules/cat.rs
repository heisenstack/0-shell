use crate::modules::utils::fix_path;
use std::fs;
use std::io::{self, Write};

pub fn cat(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return input_echo();
    }

    for file_path in args {
        if file_path == "-" || file_path == "--" {
            if let Err(_) = input_echo() {}
        } else {
            let fixed_path = fix_path(file_path);
            match read_and_print_file(&fixed_path) {
                Ok(_) => {}
                Err(error_msg) => {
                    eprintln!("{}", error_msg);
                }
            }
        }
    }

    Ok(String::new())
}

fn input_echo() -> Result<String, String> {
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                print!("{}", line);
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Error flushing stdout: {}", e);
                    break;
                }
            }
            Err(_) => break,
        }
    }

    Ok(String::new())
}

fn read_and_print_file(file_path: &str) -> Result<(), String> {
    match fs::read(file_path) {
        Ok(bytes) => {
            if let Err(e) = io::stdout().write_all(&bytes) {
                eprintln!("Error writing to stdout: {}", e);
            }
            if let Err(e) = io::stdout().flush() {
                eprintln!("Error flushing stdout: {}", e);
            }
            Ok(())
        }
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("cat: {}: No such file or directory", file_path)
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("cat: {}: Permission denied", file_path)
                }
                _ => {
                    format!("cat: {}: {}", file_path, e)
                }
            };
            Err(error_msg)
        }
    }
}
