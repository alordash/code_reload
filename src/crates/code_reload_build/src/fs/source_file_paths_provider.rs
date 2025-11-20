use std::path::{Path, PathBuf};

pub trait ISourceFilePathsProvider {
    fn provide(&self, path: &Path) -> Vec<PathBuf>;
}

pub struct SourceFilesProvider;

const RUST_FILE_EXTENSION: &'static str = "rs";

impl ISourceFilePathsProvider for SourceFilesProvider {
    fn provide(&self, path: &Path) -> Vec<PathBuf> {
        let mut source_files_paths = Vec::new();
        self.recursive_provide(path, &mut source_files_paths);
        return source_files_paths;
    }
}

impl SourceFilesProvider {
    fn recursive_provide(&self, path: &Path, source_files_paths: &mut Vec<PathBuf>) {
        for dir_entry in std::fs::read_dir(path).expect("Should be able to read all dir") {
            let inner_path = dir_entry.unwrap().path();
            if inner_path.is_file() {
                if inner_path
                    .extension()
                    .is_some_and(|ext| ext == RUST_FILE_EXTENSION)
                {
                    source_files_paths.push(inner_path);
                }
            } else {
                self.recursive_provide(&inner_path, source_files_paths);
            }
        }
    }
}
