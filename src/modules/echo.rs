pub fn echo(args: &str) -> Result<String, String> {
    if args.starts_with("-n ") {
        Ok(args[3..].to_string())
    } else if args == "-n" {
        Ok(String::new())
    } else if args.is_empty() {
        Ok("\n".to_string())
    } else {
        Ok(format!("{}\n", args))
    }
}
