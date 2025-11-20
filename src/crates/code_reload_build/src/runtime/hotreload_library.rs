use crate::runtime::{CARGO_TARGET_DIR_DEBUG, IHotreloadPayload, LibraryId};
use code_reload_core::LibraryWrapper;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

pub type LockedHotreloadLibrary<T> = LazyLock<RwLock<HotreloadLibrary<T>>>;

pub struct HotreloadLibrary<T: IHotreloadPayload> {
    pub payload: T,

    name: &'static str,
    library_wrapper: LibraryWrapper,
    dir_path: PathBuf,
    id: LibraryId,
    source_library_path: PathBuf,
    versioned_library_path: PathBuf,
}

impl<T: IHotreloadPayload> HotreloadLibrary<T> {
    pub fn load_locked(library_name: &'static str) -> RwLock<Self> {
        let hotreload_library = Self::load(library_name);
        RwLock::new(hotreload_library)
    }

    pub fn reload_locked(locked_self: &RwLock<Self>) {
        let library_name = locked_self.read().unwrap().name;
        println!("Locking '{}' for reload.", library_name);
        let mut w_self = locked_self.write().unwrap();
        let previous_versioned_library_path = w_self.versioned_library_path.clone();
        w_self.reload();
        drop(w_self);
        println!("Unlocked '{}' after reload.", library_name);

        std::fs::remove_file(&previous_versioned_library_path).unwrap();
        println!(
            "Cleared previous '{}' library file (path: '{:?}')",
            library_name, previous_versioned_library_path
        );
    }

    pub fn get_source_library_path(&self) -> PathBuf {
        self.source_library_path.clone()
    }

    fn load(library_name: &'static str) -> Self {
        let hotreload_library_id = LibraryId::new(library_name);

        let source_library_file_name = hotreload_library_id.to_source_file_name();
        let source_library_path = CARGO_TARGET_DIR_DEBUG.join(source_library_file_name);

        let versioned_library_file_name = hotreload_library_id.to_versioned_file_name();
        let versioned_library_path = CARGO_TARGET_DIR_DEBUG.join(versioned_library_file_name);

        std::fs::copy(&source_library_path, &versioned_library_path).unwrap(); // TODO - make somehow catchable in user code?

        let library_wrapper = LibraryWrapper::new(&versioned_library_path).unwrap(); // TODO - make somehow catchable in user code?
        let payload = T::load(&library_wrapper);
        let hotreload_library = Self {
            payload,
            name: library_name,
            library_wrapper,
            dir_path: CARGO_TARGET_DIR_DEBUG.to_owned(),
            id: hotreload_library_id,
            source_library_path,
            versioned_library_path,
        };
        return hotreload_library;
    }

    fn reload(&mut self) {
        let next_library_id = self.id.get_next_version();
        let next_versioned_library_file_name = next_library_id.to_versioned_file_name();
        let mut next_versioned_library_path = self.dir_path.clone();
        next_versioned_library_path.set_file_name(next_versioned_library_file_name);

        std::fs::copy(&self.source_library_path, &next_versioned_library_path).unwrap(); // TODO - make somehow catchable in user code?

        let library_wrapper = LibraryWrapper::new(&next_versioned_library_path).unwrap(); // TODO - make somehow catchable in user code?

        let payload = T::load(&library_wrapper);
        self.payload = payload;
        self.library_wrapper = library_wrapper;
        self.id = next_library_id;
        self.versioned_library_path = next_versioned_library_path;
    }
}
