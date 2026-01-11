use std::env;
use std::path::PathBuf;

/// Converts shell shortcuts like '~' into full, absolute system paths.
pub fn fix_path(path: &str) -> String {
    // Check if the path starts with the home directory shortcut '~'
    if path.starts_with("~") {
        
        // Case 1: The path is exactly "~"
        if path == "~" {
            // Replace with the value of the $HOME environment variable
            return env::var("HOME").unwrap_or(".".to_string());
            
        } 
        // Case 2: The path starts with "~/" (e.g., "~/Documents")
        else if path.starts_with("~/") {
            let home = env::var("HOME").unwrap_or(".".to_string());
            let mut expanded = PathBuf::from(home);
            
            // Append everything after the "~/" to the home directory path
            expanded.push(&path[2..]);
            
            // Return the new combined path as a string
            return expanded.to_string_lossy().into_owned();
        }
    }
    
    // If no shortcuts were found, return the path exactly as it was provided
    path.to_string()
}