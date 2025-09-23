use {
    crate::modules::utils::fix_path,
    chrono::{DateTime, Local},
    libc::{getgrgid, getpwuid},
    std::ffi::CStr,
    std::fs,
    std::os::unix::fs::FileTypeExt,
    std::os::unix::fs::{MetadataExt, PermissionsExt},
    std::path::{Path, PathBuf},
    std::time::SystemTime,
    term_size,
};

struct DirE {
    name: String,
    path: PathBuf,
}

pub fn ls(arr: &[String]) -> Result<String, String> {
    let mut tag_l = false;
    let mut tag_a = false;
    let mut tag_f = false;
    let mut vars = Vec::new();

    for op in arr {
        if op == "-" {
            return Err(format!("ls: cannot access '-': No such file or directory"));
        } else if op.starts_with('-') {
            for i in op.chars().skip(1) {
                match i {
                    'l' => tag_l = true,
                    'a' => tag_a = true,
                    'F' => tag_f = true,
                    _ => return Err("ls: invalid option".to_string()),
                }
            }
        } else {
            vars.push(fix_path(op));
        }
    }
    if vars.is_empty() {
        vars.push(".".to_string());
    }

    let mut result_parts: Vec<String> = Vec::new();
    let mut is_first_item = true;

    for var in &vars {
        let path = PathBuf::from(var);

        match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    if vars.len() > 1 {
                        if !is_first_item {
                            result_parts.push("\n".to_string());
                        }
                        result_parts.push(format!("{}:", var));
                        is_first_item = false;
                    }

                    match get_directory_entries(var, tag_a) {
                        Ok(entries) => {
                            let mut push_result: Vec<String> = Vec::new();
                            let mut total_blocks = 0;

                            for entry in entries {
                                if let Ok(meta) = fs::symlink_metadata(&entry.path) {
                                    total_blocks += meta.blocks();
                                    if let Ok(formatted_line) =
                                        format_entry(&entry.path, &entry, tag_l, tag_f)
                                    {
                                        push_result.push(formatted_line);
                                    }
                                }
                            }

                            if tag_l {
                                result_parts.push(format!("total {}", total_blocks / 2));
                                result_parts.extend(push_result);
                            } else {
                                result_parts.push(format_columns(&push_result));
                            }
                        }
                        Err(e) => {
                            result_parts.push(format!("ls: cannot access '{}': {}", var, e));
                        }
                    }
                } else {
                    if let Ok(formatted_line) = format_entry(
                        &path,
                        &DirE {
                            name: (path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()),
                            path: (PathBuf::from(var)),
                        },
                        tag_l,
                        tag_f,
                    ) {
                        result_parts.push(formatted_line);
                    }
                }
            }
            Err(e) => {
                result_parts.push(format!("ls: cannot access '{}': {}", var, e));
            }
        }
    }

    let output = if tag_l {
        result_parts.join("\n")
    } else {
        result_parts.join("\n")
    };
    if tag_l {
        return Ok(format!("{}\n", output));
    };
    Ok(output)
}

fn get_directory_entries(path: &str, tag_a: bool) -> Result<Vec<DirE>, String> {
    let mut entries: Vec<DirE> = fs::read_dir(path)
        .map_err(|e| format!("ls: cannot access '{}': {}", path, e))?
        .filter_map(|e| {
            e.ok().map(|entry| {
                let path = entry.path();
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                DirE { name, path }
            })
        })
        .collect();

    entries.sort_by(|a, b| {
        let clean_name = |name: &str| {
            name.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase()
        };
        clean_name(&a.name).cmp(&clean_name(&b.name))
    });

    entries.insert(
        0,
        DirE {
            name: ".".to_string(),
            path: PathBuf::from(path),
        },
    );
    entries.insert(
        1,
        DirE {
            name: "..".to_string(),
            path: PathBuf::from(format!("{}/..", path)),
        },
    );

    if !tag_a {
        entries.retain(|e| !e.name.starts_with('.'));
    }

    Ok(entries)
}

fn format_entry(path: &Path, entre: &DirE, tag_l: bool, tag_f: bool) -> Result<String, String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    let mut name_display = quote_name(&entre.name);

    if !tag_l {
        if tag_f {
            if metadata.is_dir() {
                name_display.push('/');
            } else if metadata.file_type().is_symlink() {
                if let Ok(target_metadata) = fs::metadata(path) {
                    if target_metadata.is_dir() {
                        name_display.push('/');
                    } else if target_metadata.file_type().is_socket() {
                        name_display.push('=');
                    } else if target_metadata.file_type().is_fifo() {
                        name_display.push('|');
                    } else if target_metadata.permissions().mode() & 0o111 != 0 {
                        name_display.push('*');
                    }
                }
            } else if metadata.file_type().is_socket() {
                name_display.push('=');
            } else if metadata.file_type().is_fifo() {
                name_display.push('|');
            } else if metadata.permissions().mode() & 0o111 != 0 {
                name_display.push('*');
            }
        }
        return Ok(name_display);
    }

    // permissions
    let mode = metadata.permissions().mode();
    let perms = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        if metadata.is_dir() {
            "d"
        } else if metadata.file_type().is_symlink() {
            "l"
        } else {
            "-"
        },
        if mode & 0o400 != 0 { "r" } else { "-" },
        if mode & 0o200 != 0 { "w" } else { "-" },
        if mode & 0o100 != 0 { "x" } else { "-" },
        if mode & 0o040 != 0 { "r" } else { "-" },
        if mode & 0o020 != 0 { "w" } else { "-" },
        if mode & 0o010 != 0 { "x" } else { "-" },
        if mode & 0o004 != 0 { "r" } else { "-" },
        if mode & 0o002 != 0 { "w" } else { "-" },
        if mode & 0o001 != 0 { "x" } else { "-" },
    );

    let mut perms_display = perms;

    if has_acl(path) {
        perms_display.push('+');
    }

    // Nlink, UID, GID, Size, Time
    let nlink = metadata.nlink();
    let get_uid = get_uid_name(metadata.uid());
    let get_gid = get_gid_name(metadata.gid());
    let size = metadata.len();

    let mtime: DateTime<Local> = DateTime::from(
        SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(metadata.mtime() as u64),
    );
    let now = Local::now();
    let six_months_secs = 60 * 60 * 24 * 30 * 6;
    let diff = now.timestamp() - metadata.mtime() as i64;
    let time_str = if diff.abs() < six_months_secs {
        mtime.format("%b %e %H:%M").to_string()
    } else {
        mtime.format("%b %e  %Y").to_string()
    };

    // Symlink target
    if metadata.file_type().is_symlink() {
        if let Ok(target) = fs::read_link(path) {
            name_display.push_str(" -> ");
            name_display.push_str(&target.to_string_lossy());
        }
    }

    // Add -F flags
    if tag_f {
        if metadata.is_dir() {
            name_display.push('/');
        } else if metadata.file_type().is_symlink() {
            if let Ok(target_metadata) = fs::metadata(path) {
                if target_metadata.is_dir() {
                    name_display.push('/');
                } else if target_metadata.file_type().is_socket() {
                    name_display.push('=');
                } else if target_metadata.file_type().is_fifo() {
                    name_display.push('|');
                } else if target_metadata.permissions().mode() & 0o111 != 0 {
                    name_display.push('*');
                }
            }
        } else if metadata.file_type().is_socket() {
            name_display.push('=');
        } else if metadata.file_type().is_fifo() {
            name_display.push('|');
        } else if metadata.permissions().mode() & 0o111 != 0 {
            name_display.push('*');
        }
    }

    Ok(format!(
        "{:<12} {:>3} {:<10} {:<10} {:>8} {} {}",
        perms_display, nlink, get_uid, get_gid, size, time_str, name_display
    ))
}

fn get_uid_name(uid: u32) -> String {
    unsafe {
        let pw = getpwuid(uid);
        if pw.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*pw).pw_name);
        name.to_string_lossy().into_owned()
    }
}

fn get_gid_name(gid: u32) -> String {
    unsafe {
        let gr = getgrgid(gid);
        if gr.is_null() {
            return gid.to_string();
        }
        let name = CStr::from_ptr((*gr).gr_name);
        name.to_string_lossy().into_owned()
    }
}

fn quote_name(name: &str) -> String {
    if name.contains('\n') {
        let mut result = String::new();
        let parts = name.split_inclusive('\n');
        for part in parts {
            if part.ends_with('\n') {
                let clean = &part[..part.len() - 1];
                if !clean.is_empty() {
                    result.push_str(&format!("'{}'", clean));
                }
                result.push_str("$'\\n'");
            } else {
                result.push_str(&format!("'{}'", part));
            }
        }
        return result;
    }

    if name
        .chars()
        .any(|c| !"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-+".contains(c))
    {
        format!("'{}'", name)
    } else {
        name.to_string()
    }
}

fn has_acl(path: &Path) -> bool {
    xattr::list(path)
        .map(|mut attrs| attrs.any(|a| a == "system.posix_acl_access"))
        .unwrap_or(false)
}

fn terminal_width() -> usize {
    if let Some((w, _)) = term_size::dimensions() {
        w
    } else if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(w) = cols.parse::<usize>() {
            return w;
        }
        80
    } else {
        80
    }
}

fn format_columns(entries: &[String]) -> String {
    let term_width = terminal_width();
    let n = entries.len();
    let mut number_row = n;
    let mut number_col = 1;
    let mut col_widths = vec![0];

    for rows in 1..=n {
        let cols = (n + rows - 1) / rows;
        let mut widths = vec![0; cols];

        for c in 0..cols {
            for r in 0..rows {
                let i = r + c * rows;
                if i < n {
                    widths[c] = widths[c].max(entries[i].len());
                }
            }
        }

        let total: usize = widths.iter().map(|w| w + 2).sum();
        if total <= term_width {
            number_row = rows;
            number_col = cols;
            col_widths = widths;
            break;
        }
    }

    let mut out = String::new();
    for r in 0..number_row {
        for c in 0..number_col {
            let i = r + c * number_row;
            if i < n {
                let pad = col_widths[c] + 2;
                out.push_str(&format!("{:<width$}", entries[i], width = pad));
            }
        }
        out.push('\n');
    }
    out
}
