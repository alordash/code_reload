use regex::Regex;
use std::cell::{LazyCell, OnceCell};
use std::io::BufRead;
use std::path::PathBuf;
use code_reload_core::library_name;

pub trait IMetadataProcessor {
    fn get_dynamic_library_path(&self) -> PathBuf;
}

pub struct MetadataProcessor;

impl IMetadataProcessor for MetadataProcessor {
    fn get_dynamic_library_path(&self) -> PathBuf {
        return Self::DYNAMIC_LIBRARY_PATH_CACHE
            .get_or_init(|| self.eager_get_dynamic_library_path())
            .clone();
    }
}

impl MetadataProcessor {
    const LIB_SECTION_NAME: &'static str = "[lib]";
    const NAME_PARAMETER_REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r#"^\s*name\s*=\s*"(\w+)""#).unwrap());

    const DYNAMIC_LIBRARY_PATH_CACHE: OnceCell<PathBuf> = OnceCell::new();

    fn eager_get_dynamic_library_path(&self) -> PathBuf {
        let dynamic_library_name = std::env::var("CARGO_MANIFEST_PATH")
            .ok()
            .map(|x| self.get_crate_lib_name(x))
            .flatten()
            .or_else(|| std::env::var("CARGO_PKG_NAME").ok().map(|crate_name| crate_name.replace('-', "_")))
            .expect("Unable to determine dynamic library name. It must be determined either using [lib] section and 'name' parameter in crate manifest, or using crate name.");

        // TODO - make file extension platform agnostic
        let dynamic_library_path = [
            ".",
            &library_name::create(&dynamic_library_name),
        ]
        .iter()
        .collect();

        return dynamic_library_path;
    }

    fn get_crate_lib_name(&self, manifest_path: String) -> Option<String> {
        let manifest_file = std::fs::OpenOptions::new()
            .read(true)
            .open(&manifest_path)
            .expect(&format!(
                "Unable to read manifest file located at '{}'.",
                manifest_path
            ));
        let mut manifest_lines = std::io::BufReader::new(manifest_file).lines();

        while let Some(Ok(line)) = manifest_lines.next() {
            if line.trim_start().starts_with(Self::LIB_SECTION_NAME) {
                break;
            }
        }

        while let Some(Ok(line)) = manifest_lines.next() {
            if line.starts_with('[') {
                break;
            }
            if let Some(name_parameter) = Self::NAME_PARAMETER_REGEX.captures(&line) {
                let crate_lib_name = name_parameter[1].to_owned();
                return Some(crate_lib_name);
            }
        }

        return None;
    }
}
