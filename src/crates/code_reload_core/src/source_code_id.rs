use crate::constants;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

pub struct SourceCodeId {
    relative_file_path: PathBuf,
    line: usize,
    column: usize,
}

impl SourceCodeId {
    pub fn new(file_path: &Path, line: usize, column: usize) -> Self {
        let relative_file_path =
            merge_file_and_manifest_paths(file_path, &*constants::MANIFEST_DIR);
        Self {
            relative_file_path,
            line,
            column,
        }
    }
}

impl Display for SourceCodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{:?}':{},{}",
            self.relative_file_path, self.line, self.column
        )
    }
}

pub fn merge_file_and_manifest_paths(file_path: &Path, manifest_path: &Path) -> PathBuf {
    let mut manifest_parts = manifest_path.iter();

    let file_parent = file_path.iter().next().unwrap();
    let mut merged = PathBuf::new();
    while let Some(manifest_part) = manifest_parts.next() {
        merged.push(manifest_part);
        if manifest_part == file_parent {
            break;
        }
    }
    let mut file_path_parts = file_path.iter().skip(1);
    while let Some(manifest_part) = manifest_parts.next()
        && let Some(file_path_part) = file_path_parts.next()
        && manifest_part == file_path_part
    {
        merged.push(manifest_part);
    }
    if let Some(unexpected_manifest_part) = manifest_parts.next() {
        let mut remaining_manifest_parts = PathBuf::new();
        remaining_manifest_parts.push(unexpected_manifest_part);
        remaining_manifest_parts.extend(manifest_parts);
        panic!(
            "Manifest path should've been emptied, instead this remains: '{:?}'",
            remaining_manifest_parts
        );
    }
    merged.extend(file_path_parts);

    return merged;
}
