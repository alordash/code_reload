use code_reload_core::LibraryWrapper;

pub trait IHotreloadPayload {
    fn load(library_wrapper: &LibraryWrapper) -> Self;
}
