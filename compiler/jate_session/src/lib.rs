mod source;

pub use source::SourceFile;

use jate_error::Diagnostic;
use std::path::PathBuf;

/// Global compiler context
/// Keep all source files and errors
pub struct Session {
    sources: Vec<SourceFile>,
    diagnostics: Vec<Diagnostic>,
    class_path: Vec<PathBuf>,
}
