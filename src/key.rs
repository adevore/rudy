pub trait Key: Copy + PartialEq + Ord + Default {
    type Bytes: AsRef<[u8]>;
    const SIZE: usize;
    fn into_bytes(self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;
}


macro_rules! impl_key {
    ($type:ident, $size:expr) => {
        impl Key for $type {
            type Bytes = [u8; $size];
            const SIZE: usize = $size;
            fn into_bytes(self) -> Self::Bytes {
                unsafe {
                    ::std::mem::transmute(self.to_be())
                }
            }

            fn from_bytes(bytes: Self::Bytes) -> Self {
                let integer = unsafe {
                    ::std::mem::transmute(bytes)
                };
                $type::from_be(integer)
            }
        }
    }
}

impl_key!(u8, 1);
impl_key!(u16, 2);
impl_key!(u32, 4);
impl_key!(u64, 8);
#[cfg(feature = "i128")]
impl_key!(u128, 16);
#[cfg(target_pointer_width = "32")]
impl_key!(usize, 4);
#[cfg(target_pointer_width = "64")]
impl_key!(usize, 8);
