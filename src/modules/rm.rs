use std::fs;
use std::path::{Path};
use crate::modules::utils::fix_path;

pub fn rm(args: &[String]) -> Result<String, String> {
    // Ensure the user provided at least one file or flag
    if args.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let mut recursive = false;
    let mut paths = Vec::new();

    // 1. Parse arguments to separate flags (like -r) from file paths
    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            paths.push(fix_path(arg));
        }
    }

    // Verify there are actually files to delete after parsing flags
    if paths.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let mut messages = Vec::new();

    // 2. Iterate through each path provided
    for path_str in paths {
        let path = Path::new(&path_str);

        // Safety check: Prevent deleting '.' (current dir) or '..' (parent dir)
        if path_str == "." || path_str == ".." {
            messages.push(format!("rm: refusing to remove '{}'", path_str));
            continue;
        }

        // Check if the file/folder actually exists using symlink metadata
        if fs::symlink_metadata(path).is_err() {
            messages.push(format!("rm: cannot remove '{}': No such file or directory", path_str));
            continue;
        }

        // 3. Handle deletion logic based on file type
        if path.is_dir() {
            if recursive {
                // Delete directory and everything inside it
                if let Err(e) = fs::remove_dir_all(path) {
                    messages.push(format!("rm: failed to remove directory '{}': {}", path_str, e));
                }
            } else {
                // Standard 'rm' fails on directories without the -r flag
                messages.push(format!("rm: cannot remove '{}': Is a directory", path_str));
            }
        } else {
            // Standard file deletion
            if let Err(e) = fs::remove_file(path) {
                messages.push(format!("rm: failed to remove file '{}': {}", path_str, e));
            }
        }
    }

    // Return all collected error messages, or an empty success string
    if messages.is_empty() {
        Ok(String::new())
    } else {
        Ok(messages.join("\n") + "\n")
    }
}