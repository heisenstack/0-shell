use crate::modules::utils::fix_path;
use std::fs;

pub fn mkdir(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }
    let mut errors = Vec::new();

    for dir_name in args {
        let fixed_path = fix_path(dir_name);
        match fs::create_dir(&fixed_path) {
            Ok(_) => {}
            Err(e) => {
                let error_msg = match e.kind() {
                    std::io::ErrorKind::AlreadyExists => {
                        format!(
                            "mkdir: cannot create directory '{}': File exists",
                            fixed_path
                        )
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        format!(
                            "mkdir: cannot create directory '{}': Permission denied",
                            fixed_path
                        )
                    }
                    std::io::ErrorKind::NotFound => {
                        format!(
                            "mkdir: cannot create directory '{}': No such file or directory",
                            fixed_path
                        )
                    }
                    _ => {
                        format!("mkdir: cannot create directory '{}': {}", fixed_path, e)
                    }
                };
                errors.push(error_msg);
            }
        }
    }

    if errors.is_empty() {
        Ok(String::new())
    } else {
        Err(errors.join("\n"))
    }
}
