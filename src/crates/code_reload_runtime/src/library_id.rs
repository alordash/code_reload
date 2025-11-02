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
        format!("{}.dll", self.name)
    }

    pub fn to_versioned_file_name(&self) -> String {
        format!("{}_v{}.dll", self.name, self.version)
    }

    pub fn get_next_version(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version + 1,
        }
    }
}
