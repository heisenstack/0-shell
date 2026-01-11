use crate::modules::utils::fix_path;
use std::env;
use std::path::{Path};

// Global variable to store the previous directory for 'cd -'
static mut OLDPWD: Option<String> = None;

pub fn cd(args: &[String]) -> Result<String, String> {
    // Only 0 or 1 arguments allowed
    if args.len() > 1 {
        return Err("cd: too many arguments".to_string());
    }

    // Get current path before moving (used to update OLDPWD later)
    let current_dir = match env::current_dir() {
        Ok(path) => Some(path.to_string_lossy().into_owned()),
        Err(_) => None,
    };

    // Determine where we are going
    let target = if args.is_empty() {
        // 'cd' defaults to HOME
        env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
    } else if args[0] == "-" {
        // 'cd -' switches to the previous directory
        unsafe {
            let oldpwd_ptr = std::ptr::addr_of!(OLDPWD);
            match (*oldpwd_ptr).clone() {
                Some(old_dir) => old_dir,
                None => return Err("cd: OLDPWD not set".to_string()),
            }
        }
    } else {
        // Standard path logic
        let raw_target = fix_path(&args[0]);

        // Fix for when the current directory was deleted while the shell was in it
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

    // Validation: Does it exist and is it a directory?
    if !path.exists() {
        return Err(format!("cat: {}: No such file or directory", target));
    }
    if !path.is_dir() {
        return Err(format!("cd: {}: Not a directory", target));
    }

    // Perform the directory change
    if let Err(e) = env::set_current_dir(path) {
        return Err(format!("cd: failed to change directory: {}", e));
    }

    // Update OLDPWD so 'cd -' works next time
    if let Some(prev_dir) = current_dir {
        unsafe {
            OLDPWD = Some(prev_dir);
        }
    }

    // 'cd -' prints the new directory path; others return an empty string
    if !args.is_empty() && args[0] == "-" {
        Ok(format!("{}\n", target))
    } else {
        Ok("".to_string())
    }
}