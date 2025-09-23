use crate::modules::utils::fix_path;
use std::fs;
use std::path::{Path, PathBuf};

pub fn mv(args: &[String]) -> Result<String, String> {
    if args.len() != 2 {
        return Err("mv: missing file operand or destination".to_string());
    }

    let fixed_src = fix_path(&args[0]);
    let fixed_dest = fix_path(&args[1]); 

    let src = Path::new(&fixed_src);
    let dest = Path::new(&fixed_dest);

    if !src.exists() {
        return Err(format!(
            "mv: cannot stat '{}': No such file or directory",
            fixed_src
        ));
    }

    let final_dest: PathBuf = if dest.is_dir() {
        dest.join(src.file_name().ok_or("mv: invalid source filename")?)
    } else {
        dest.to_path_buf()
    };

    match (src.canonicalize(), final_dest.canonicalize()) {
        (Ok(canonical_source), Ok(canonical_dest)) => {
            if canonical_source == canonical_dest {
                return Err(format!(
                    "mv: '{}' and '{}' are the same file",
                    fixed_src,
                    final_dest.display()
                ));
            }
        }
        _ => {
            if src == final_dest {
                return Err(format!(
                    "mv: '{}' and '{}' are the same file",
                    fixed_src,
                    final_dest.display()
                ));
            }
        }
    }

    if let Err(e) = fs::rename(src, &final_dest) {
        return Err(format!("mv: failed to move '{}': {}", fixed_src, e));
    }

    Ok("".to_string())
}
