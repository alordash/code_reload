use std::path::PathBuf;

pub(crate) trait ISourceFilesProvider {
    fn provide(&self) -> Vec<PathBuf>;
}