use crate::modules::utils::fix_path;
use std::env;
use std::path::{Path};

static mut OLDPWD: Option<String> = None;

pub fn cd(args: &[String]) -> Result<String, String> {
    if args.len() > 1 {
        return Err("cd: too many arguments".to_string());
    }

    let current_dir = match env::current_dir() {
        Ok(path) => Some(path.to_string_lossy().into_owned()),
        Err(_) => None,
    };

    let target = if args.is_empty() {
        env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
    } else if args[0] == "-" {
        unsafe {
            let oldpwd_ptr = std::ptr::addr_of!(OLDPWD);
            match (*oldpwd_ptr).clone() {
                Some(old_dir) => old_dir,
                None => return Err("cd: OLDPWD not set".to_string()),
            }
        }
    } else {
        let raw_target = fix_path(&args[0]);

        if current_dir.is_none() && !Path::new(&raw_target).is_absolute() {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            eprintln!(
                "cd: current directory has been removed, redirecting to home for relative path resolution"
            );
            if let Err(e) = env::set_current_dir(&home) {
                return Err(format!("cd: failed to change to home directory: {}", e));
            }
        }

        raw_target
    };

    let path = Path::new(&target);

    if !path.exists() {
        return Err(format!("cd: {}: No such file or directory", target));
    }
    if !path.is_dir() {
        return Err(format!("cd: {}: Not a directory", target));
    }

    if let Err(e) = env::set_current_dir(path) {
        return Err(format!("cd: failed to change directory: {}", e));
    }

    if let Some(prev_dir) = current_dir {
        unsafe {
            OLDPWD = Some(prev_dir);
        }
    }

    if !args.is_empty() && args[0] == "-" {
        Ok(format!("{}\n", target))
    } else {
        Ok("".to_string())
    }
}
