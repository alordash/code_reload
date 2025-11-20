use std::path::PathBuf;

pub struct SourceCodeId {
    pub file_path: PathBuf,
    pub row: usize,
    pub column: usize
}