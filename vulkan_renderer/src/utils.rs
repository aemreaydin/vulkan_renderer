unsafe fn to_u8_slice<T: Sized>(data: &T) -> &[u8] {
    std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of::<T>())
}

pub trait U8Slice {
    fn as_u8_slice(&self) -> &[u8];
}
impl<T: Sized> U8Slice for T {
    fn as_u8_slice(&self) -> &[u8] {
        unsafe { to_u8_slice(self) }
    }
}
