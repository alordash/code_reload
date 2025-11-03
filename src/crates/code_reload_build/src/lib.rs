mod fs;

mod executable;
mod library;
pub use executable::*;
pub use library::*;

use std::path::{Path, PathBuf};

pub fn get_all_rust_file_paths(path: &Path) -> Vec<PathBuf> {
    let mut rust_file_paths = Vec::new();
    recursive_get_all_rust_file_paths(path, &mut rust_file_paths);
    return rust_file_paths;
}

const RUST_FILE_EXTENSION: &'static str = "rs";

fn recursive_get_all_rust_file_paths(path: &Path, rust_file_paths: &mut Vec<PathBuf>) {
    for dir_entry in std::fs::read_dir(path).expect("Should be able to read all dir") {
        let inner_path = dir_entry.unwrap().path();
        if inner_path.is_file() {
            if inner_path
                .extension()
                .is_some_and(|ext| ext == RUST_FILE_EXTENSION)
            {
                rust_file_paths.push(inner_path);
            }
        } else {
            recursive_get_all_rust_file_paths(&inner_path, rust_file_paths);
        }
    }
}
