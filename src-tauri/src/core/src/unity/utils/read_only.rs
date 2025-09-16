#[repr(transparent)]
pub struct ReadOnly<T>(#[doc = "The encapsulated value."] T);

impl<T> ReadOnly<T> {
    pub fn new(inner: T) -> Self {
        ReadOnly(inner)
    }

    pub fn take(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for ReadOnly<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<T> Send for ReadOnly<T> {}
unsafe impl<T> Sync for ReadOnly<T> {}
