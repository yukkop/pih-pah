pub fn module_cat(string: &str) -> &str {
    string.splitn(3, ':').nth(2).unwrap_or(string.clone())
}