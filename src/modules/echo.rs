pub fn echo(args: &str) -> Result<String, String> {
    // Case 1: Handle "-n " prefix to suppress the trailing newline
    if args.starts_with("-n ") {
        // Return only the text following the "-n "
        Ok(args[3..].to_string())
    } 
    // Case 2: Handle exactly "-n" with no subsequent text
    else if args == "-n" {
        Ok(String::new())
    } 
    // Case 3: Handle empty input (just a 'newline' result)
    else if args.is_empty() {
        Ok("\n".to_string())
    } 
    // Case 4: Standard echo (print the text followed by a newline)
    else {
        Ok(format!("{}\n", args))
    }
}