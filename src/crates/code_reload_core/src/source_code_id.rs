use crate::constants;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

// TODO - add interface
#[derive(Clone)]
pub struct SourceCodeId {
    relative_file_path: PathBuf,
    line: usize,
    column: usize,
}

impl SourceCodeId {
    pub fn new(relative_file_path: PathBuf, line: usize, column: usize) -> Self {
        Self {
            relative_file_path,
            line,
            column,
        }
    }

    pub fn get_fn_ident(&self, source_fn_ident: &str) -> String {
        let ident = self
            .relative_file_path
            .iter()
            .map(|x| x.to_str().unwrap())
            .chain(vec![
                self.line.to_string().as_ref(),
                self.column.to_string().as_ref(),
                source_fn_ident,
            ])
            .collect::<Vec<_>>()
            .join("_");

        return ident;
    }

    pub fn get_path(&self) -> PathBuf {
        let path = self
            .relative_file_path
            .join(format!("{}.{}", self.line, self.column));
        return path;
    }

    pub fn get_module(&self) -> String {
        let module = std::iter::once("crate")
            .chain(
                self.relative_file_path
                    .iter()
                    // skip first two because they correspond to "crate" (like "src/lib")
                    .skip(2)
                    .map(|x| x.to_str().unwrap()),
            )
            .collect::<Vec<_>>()
            .join("::");
        return module;
    }

    // TODO - replace with service?
    pub fn get_source_code_relative_file_path(file_path: &Path) -> PathBuf {
        let manifest_dir = &*constants::MANIFEST_DIR;
        let absolute_file_path =
            merge_file_and_manifest_paths(&file_path.with_extension(""), manifest_dir);
        let mut parent_iter = manifest_dir.iter().peekable();
        let mut child_iter = absolute_file_path.iter().peekable();
        while let Some(parent_part) = parent_iter.peek()
            && let Some(child_part) = child_iter.peek()
            && parent_part == child_part
        {
            parent_iter.next();
            child_iter.next();
        }
        let relative_file_path = child_iter
            .map(|x| Self::normalize_path_part(x.to_str().unwrap()))
            .collect();
        return relative_file_path;
    }

    fn normalize_path_part(path_part: &str) -> String {
        let normalized_path_part = path_part.replace(|x| !char::is_alphanumeric(x), "_");

        return normalized_path_part;
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
