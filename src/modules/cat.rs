use crate::modules::utils::fix_path;
use std::fs;
use std::io::{self, Write};

/// The main entry point for the 'cat' command logic.
/// Accepts a slice of strings representing file paths or special flags.
pub fn cat(args: &[String]) -> Result<String, String> {
    // If no arguments are provided, default to echoing standard input (stdin)
    if args.is_empty() {
        return input_echo();
    }

    for file_path in args {
        // Handle "-" or "--" as aliases for reading from standard input
        if file_path == "-" || file_path == "--" {
            // If stdin reading fails, we ignore the error and move to the next argument
            if let Err(_) = input_echo() {}
        } else {
            // Resolve path issues (e.g., home directory expansion or absolute pathing)
            let fixed_path = fix_path(file_path);
            
            // Attempt to read and print the file; errors are printed to stderr but don't halt the loop
            match read_and_print_file(&fixed_path) {
                Ok(_) => {}
                Err(error_msg) => {
                    eprintln!("{}", error_msg);
                }
            }
        }
    }

    // Return an empty string on success to satisfy the return type
    Ok(String::new())
}

/// Reads from standard input and prints it back to standard output line by line.
/// Continues until an EOF (Ctrl+D) or an error occurs.
fn input_echo() -> Result<String, String> {
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                // End of File reached (0 bytes read)
                break;
            }
            Ok(_) => {
                print!("{}", line);
                // Ensure the output is displayed immediately
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Error flushing stdout: {}", e);
                    break;
                }
            }
            Err(_) => break, // Exit loop on read error
        }
    }

    Ok(String::new())
}

/// Reads the entire content of a file and writes it directly to stdout.
/// Optimized for binary and text files by reading raw bytes.
fn read_and_print_file(file_path: &str) -> Result<(), String> {
    match fs::read(file_path) {
        Ok(bytes) => {
            // Write raw bytes to handle non-UTF-8 files gracefully
            if let Err(e) = io::stdout().write_all(&bytes) {
                eprintln!("Error writing to stdout: {}", e);
            }
            // Ensure all data is pushed to the terminal
            if let Err(e) = io::stdout().flush() {
                eprintln!("Error flushing stdout: {}", e);
            }
            Ok(())
        }
        Err(e) => {
            // Map OS errors to user-friendly 'cat' style error messages
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