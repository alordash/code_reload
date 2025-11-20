use crate::LibraryWrapper;

pub trait IHotreloadPayload {
    fn load(library_wrapper: &LibraryWrapper) -> Self;
}