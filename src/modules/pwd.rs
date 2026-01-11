use std::env;

pub fn pwd(args: &[String]) -> Result<String, String> {
    // pwd typically doesn't accept arguments
    if !args.is_empty() {
        return Err("pwd: too many arguments".to_string());
    }

    // Ask the operating system for the current working directory
    match env::current_dir() {
        Ok(path) => match path.to_str() {
            // Success: convert the path to a string and add a newline
            Some(path_str) => Ok(format!("{}\n", path_str)),
            // Handle rare cases where the path isn't valid UTF-8
            None => Err("pwd: current directory path contains invalid UTF-8".to_string()),
        },
        Err(e) => {
            // Handle specific case where the folder you are 'in' was deleted
            if e.kind() == std::io::ErrorKind::NotFound {
                Err("pwd: current directory has been removed".to_string())
            } else {
                Err(format!("pwd: error getting current directory: {}", e))
            }
        }
    }
}