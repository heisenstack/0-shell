use {
    crate::modules::cat::cat, crate::modules::cd::cd, crate::modules::cp::cp,
    crate::modules::echo::echo, crate::modules::ls::ls, crate::modules::mkdir::mkdir,
    crate::modules::mv::mv, crate::modules::pwd::pwd, crate::modules::rm::rm,
};

pub fn prossess(value: Vec<String>) -> Result<String, String> {
    let cmd = value[0].clone();
    match cmd.as_str() {
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
        "exit" => Ok("exit".to_string()),
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}
