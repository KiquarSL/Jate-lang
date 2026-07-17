use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpanKind {
    /// `+++`
    Add,
    /// `---`
    Remove,
    /// `^^^`
    #[default]
    Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub len: u32,
    pub kind: SpanKind,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "start: {}, len: {}", self.start, self.len)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Help {
    message: String,
    span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    pub err_code: String,
    pub span: Span,
    pub helps: Vec<Help>,
}

/** Macros for fast make diagnostic

# Arguments

1. Message, error code, span, helps
2. Message, error code, span

# Example

```
use jate_error::{diag, span, SpanKind, Diagnostic};

let diagnostic = diag!(
    "E0001",
    span!(10, 3, SpanKind::Point),
    "Undefined variable"
);
```
*/
#[macro_export]
macro_rules! diag {
    ($err_code: expr, $span: expr, $helps: expr, $($arg:tt)*) => {
        Diagnostic {
            message: format!($($arg)*),
            err_code: $err_code.into(),
            span: $span,
            helps: $helps,
        }
    };
    ($err_code: expr, $span: expr, $($arg:tt)*) => {
        diag!($err_code, $span, vec![], $($arg)*)
    };
}

/** Macros for fast make span

# Arguments

1. Position, length, span kind
2. Position, length (Using default kind)
*/
#[macro_export]
macro_rules! span {
    ($start:expr, $len:expr, $kind:expr) => {
        jate_error::Span {
            start: $start,
            len: $len,
            kind: $kind,
        }
    };
    ($start:expr, $len:expr) => {
        span!($start, $len, jate_error::SpanKind::default())
    };
}

/** Macros for fast make help

# Arguments

Message, span
*/
#[macro_export]
macro_rules! help {
    ($span: expr, $($arg:tt)*) => {
        Help {
            message: format!($($arg)*),
            span: $span,
        }
    };
}
