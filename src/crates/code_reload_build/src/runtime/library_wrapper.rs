// This struct is used to relieve user-code from directly depending on `libloading` crate.
pub struct LibraryWrapper {
    inner: libloading::Library,
}

impl LibraryWrapper {
    pub fn new(inner: libloading::Library) -> Self {
        Self { inner }
    }

    pub fn get<T: Copy>(&self, symbol: &[u8]) -> Result<T, libloading::Error> {
        unsafe { self.inner.get(symbol).map(|x| *x) }
    }
}
