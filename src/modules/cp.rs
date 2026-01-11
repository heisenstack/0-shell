use std::fs;
use std::path::{Path};
use crate::modules::utils::fix_path;

/// Main copy function: handles one or more sources and a destination
pub fn cp(args: &[String]) -> Result<String, String> {
    // Ensure we have at least one source and one destination
    if args.len() < 2 {
        return Err("cp: missing file operand\nUsage: cp SOURCE... DEST".to_string());
    }

    // Split args: the last item is the destination, everything else is a source
    let (sources, destination) = args.split_at(args.len() - 1);
    let dest = fix_path(&destination[0]);
    let dest_path = Path::new(&dest);

    // If multiple sources are provided, the destination MUST be an existing directory
    if sources.len() > 1 && !dest_path.is_dir() {
        return Err(format!("cp: target '{}' is not a directory", dest));
    }

    let mut errors = Vec::new();

    // Iterate through all source files and copy them one by one
    for source in sources {
        let fixed_source = fix_path(source);
        match copy_single_file(&fixed_source, &dest) {
            Ok(_) => {}
            Err(error_msg) => errors.push(error_msg),
        }
    }

    // Return success if no errors occurred, otherwise join and return all errors
    if errors.is_empty() {
        Ok(String::new())
    } else {
        Err(errors.join("\n"))
    }
}

/// Logic for copying a single file to a destination
fn copy_single_file(source: &str, destination: &str) -> Result<(), String> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    // Check if source exists
    if !source_path.exists() {
        return Err(format!(
            "cp: cannot stat '{}': No such file or directory",
            source
        ));
    }

    // This implementation only handles files, not directories (no -r flag)
    if source_path.is_dir() {
        return Err(format!(
            "cp: -r not specified; omitting directory '{}'",
            source
        ));
    }

    // Determine the final destination path
    let final_dest = if dest_path.is_dir() {
        // If destination is a folder, append the source filename to it
        match source_path.file_name() {
            Some(file_name) => dest_path.join(file_name),
            None => return Err(format!("cp: invalid source file name '{}'", source)),
        }
    } else {
        // If destination is a file path, use it directly
        dest_path.to_path_buf()
    };

    // Safety: Prevent copying a file onto itself (e.g., 'cp file.txt file.txt')
    match (source_path.canonicalize(), final_dest.canonicalize()) {
        (Ok(canonical_source), Ok(canonical_dest)) => {
            if canonical_source == canonical_dest {
                return Err(format!(
                    "cp: '{}' and '{}' are the same file",
                    source,
                    final_dest.display()
                ));
            }
        }
        _ => {
            // Fallback check if canonicalization fails
            if source_path == final_dest {
                return Err(format!(
                    "cp: '{}' and '{}' are the same file",
                    source,
                    final_dest.display()
                ));
            }
        }
    }

    // Execute the actual file copy
    match fs::copy(source, &final_dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Map IO errors to user-friendly messages
            let error_msg = match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!(
                        "cp: cannot create regular file '{}': Permission denied",
                        final_dest.display()
                    )
                }
                std::io::ErrorKind::NotFound => {
                    format!(
                        "cp: cannot create regular file '{}': No such file or directory",
                        final_dest.display()
                    )
                }
                _ => {
                    format!(
                        "cp: cannot copy '{}' to '{}': {}",
                        source,
                        final_dest.display(),
                        e
                    )
                }
            };
            Err(error_msg)
        }
    }
}