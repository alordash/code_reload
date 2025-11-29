use code_reload_core::constants;

pub type LibraryVersion = u32;

pub struct LibraryId {
    name: String,
    version: LibraryVersion,
}

impl LibraryId {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
            version: 0,
        }
    }

    pub fn to_source_file_name(&self) -> String {
        format!("{}.{}", self.name, constants::DYNAMIC_LIBRARY_EXTENSION)
    }

    pub fn to_versioned_file_name(&self) -> String {
        format!("{}_v{}.{}", self.name, self.version, constants::DYNAMIC_LIBRARY_EXTENSION)
    }

    pub fn get_next_version(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version + 1,
        }
    }
}
