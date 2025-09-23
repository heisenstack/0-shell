use std::fs;
use std::path::{Path};
use crate::modules::utils::fix_path;

pub fn rm(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let mut recursive = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            paths.push(fix_path(arg));
        }
    }

    if paths.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let mut messages = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);

        if path_str == "." || path_str == ".." {
            messages.push(format!("rm: refusing to remove '{}'", path_str));
            continue;
        }

        if fs::symlink_metadata(path).is_err() {
            messages.push(format!("rm: cannot remove '{}': No such file or directory", path_str));
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    messages.push(format!("rm: failed to remove directory '{}': {}", path_str, e));
                }
            } else {
                messages.push(format!("rm: cannot remove '{}': Is a directory", path_str));
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                messages.push(format!("rm: failed to remove file '{}': {}", path_str, e));
            }
        }
    }

    if messages.is_empty() {
        Ok(String::new())
    } else {
        Ok(messages.join("\n") + "\n")
    }
}
