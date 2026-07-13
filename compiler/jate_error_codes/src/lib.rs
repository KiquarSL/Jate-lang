/// Get error help by name
pub fn get_error(code: &str) -> &str {
    match code {
        "E0001" => include_str!("../errors/E0001.md"),
        "E0002" => include_str!("../errors/E0002.md"),
        "E0003" => include_str!("../errors/E0003.md"),
        "E0004" => include_str!("../errors/E0003.md"),
        _ => "Unknown error",
    }
}
