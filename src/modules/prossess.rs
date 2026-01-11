use {
    crate::modules::cat::cat, crate::modules::cd::cd, crate::modules::cp::cp,
    crate::modules::echo::echo, crate::modules::ls::ls, crate::modules::mkdir::mkdir,
    crate::modules::mv::mv, crate::modules::pwd::pwd, crate::modules::rm::rm,
};

/// Takes a vector of strings (command + arguments) and routes them to the right function
pub fn prossess(value: Vec<String>) -> Result<String, String> {
    // The first element is the command name (e.g., "ls")
    let cmd = value[0].clone();

    // Match the command name to its corresponding logic
    match cmd.as_str() {
        // For most commands, pass everything after the first element as arguments
        "cp" => match cp(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "mkdir" => match mkdir(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "pwd" => match pwd(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "cat" => match cat(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "echo" => {
            // Echo is unique: it joins all arguments back into a single string
            let args = if value.len() > 1 {
                value[1..].join(" ")
            } else {
                String::new()
            };
            match echo(&args) {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            }
        }
        "rm" => match rm(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "cd" => match cd(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "mv" => match mv(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        "ls" => match ls(&value[1..]) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        },
        // Handle the shell exit command
        "exit" => Ok("exit".to_string()),
        // Return an error if the user typed something unknown
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}