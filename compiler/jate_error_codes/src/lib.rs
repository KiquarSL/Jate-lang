/// Get error help by name
pub fn get_error(code: &str) -> &str {
    match code {
        "E0001" => include_str!("../errors/E0001.md"),
        _ => "Unknown error",
    }
}
