use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub struct SourceCodeId {
    pub file_path: PathBuf,
    pub line: usize,
    pub column: usize
}

impl Display for SourceCodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{:?}':{},{}", self.file_path, self.line, self.column)
    }
}