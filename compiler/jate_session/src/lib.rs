mod source;

pub use source::SourceFile;

use jate_error::Diagnostic;

pub struct Session {
    sources: Vec<SourceFile>,
    diagnostics: Vec<Diagnostic>,
}
