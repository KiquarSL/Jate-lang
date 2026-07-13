/// Kepp source file name, source text and starts of lines
#[derive(Debug)]
pub struct SourceFile {
    name: String,
    src: String,
    lines: Vec<u32>,
}

/// Find starts lines of source
fn split_source(src: &str) -> Vec<u32> {
    let mut lines = vec![0];
    let mut i = 1;
    for c in src.chars() {
        if c == '\n' {
            lines.push(i);
        }
        i += 1;
    }
    return lines;
}

impl SourceFile {
    pub fn new(name: String, src: String) -> Self {
        Self {
            name,
            src: src.clone(),
            lines: split_source(&src),
        }
    }

    /// Get line by line number
    /// Return clone of line
    pub fn get_line(self, line_number: usize) -> String {
        let src_line = &self.src[line_number..];
        let end = src_line.find('\n').unwrap_or(src_line.len() - 1);
        return src_line[..end].to_string();
    }
}
