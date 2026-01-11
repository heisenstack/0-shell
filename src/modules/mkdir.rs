use crate::modules::utils::fix_path;
use std::fs;

pub fn mkdir(args: &[String]) -> Result<String, String> {
    // Ensure at least one directory name is provided
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }
    
    let mut errors = Vec::new();

    // Iterate through all provided directory names
    for dir_name in args {
        // Resolve the path (e.g., handling relative paths or home shortcuts)
        let fixed_path = fix_path(dir_name);
        
        // Attempt to create the directory
        match fs::create_dir(&fixed_path) {
            Ok(_) => {} // Success: do nothing and continue to next
            Err(e) => {
                // Map system errors to user-friendly messages
                let error_msg = match e.kind() {
                    std::io::ErrorKind::AlreadyExists => {
                        format!("mkdir: cannot create directory '{}': File exists", fixed_path)
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        format!("mkdir: cannot create directory '{}': Permission denied", fixed_path)
                    }
                    std::io::ErrorKind::NotFound => {
                        // This usually happens if the parent directory doesn't exist
                        format!("mkdir: cannot create directory '{}': No such file or directory", fixed_path)
                    }
                    _ => {
                        format!("mkdir: cannot create directory '{}': {}", fixed_path, e)
                    }
                };
                errors.push(error_msg);
            }
        }
    }

    // If any directories failed to be created, return all collected errors
    if errors.is_empty() {
        Ok(String::new())
    } else {
        Err(errors.join("\n"))
    }
}