#[derive(Debug)]
pub struct SourceFile {
    name: String,
    src: String,
    lines: Vec<u32>,
}

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
}
