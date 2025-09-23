use std::env;
use std::path::PathBuf;

pub fn fix_path(path: &str) -> String {
    if path.starts_with("~") {
        if path == "~" {
            return env::var("HOME").unwrap_or(".".to_string());
        } else if path.starts_with("~/") {
            let home = env::var("HOME").unwrap_or(".".to_string());
            let mut expanded = PathBuf::from(home);
            expanded.push(&path[2..]);
            return expanded.to_string_lossy().into_owned();
        }
    }
    path.to_string()
}
