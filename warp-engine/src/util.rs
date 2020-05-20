use std::mem::size_of;

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl<T> IntoBytes for Vec<T> {
    fn into_bytes(self) -> Vec<u8> {
        let len = size_of::<T>() * self.len();
        let slice = self.into_boxed_slice();

        unsafe { Vec::<u8>::from_raw_parts(Box::into_raw(slice) as _, len, len) }
    }
}
