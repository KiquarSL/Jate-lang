#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpanKind {
    /// `+++`
    Add,
    /// `---`
    Remove,
    /// `^^^`
    Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub len: u32,
    pub kind: SpanKind,
}

#[derive(Debug, Clone)]
pub struct Help {
    message: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub err_code: String,
    pub span: Span,
    pub helps: Vec<Help>,
}
