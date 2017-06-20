use std::fmt::Debug;

pub trait Key: Copy + PartialEq + Ord + Default + Debug {
    type Bytes: AsRef<[u8]>;
    const SIZE: usize;
    fn into_bytes(self) -> Self::Bytes;
    fn from_bytes(bytes: Self::Bytes) -> Self;
}


macro_rules! impl_unsigned_key {
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

impl_unsigned_key!(u8, 1);
impl_unsigned_key!(u16, 2);
impl_unsigned_key!(u32, 4);
impl_unsigned_key!(u64, 8);
#[cfg(feature = "i128")]
impl_unsigned_key!(u128, 16);
#[cfg(target_pointer_width = "32")]
impl_unsigned_key!(usize, 4);
#[cfg(target_pointer_width = "64")]
impl_unsigned_key!(usize, 8);

// Signed integer keys are represented in the JPM as shifted values
// to allow sorting to work correctly with two's complement
// representations. Otherwise, the ones in negative numbers would
// cause them to come after positive numbers.
macro_rules! impl_signed_key {
    ($type:ident, $size:expr) => {
        impl Key for $type {
            type Bytes = [u8; $size];
            const SIZE: usize = $size;
            fn into_bytes(self) -> Self::Bytes {
                let shifted = self.wrapping_sub($type::min_value());
                unsafe {
                    ::std::mem::transmute(shifted.to_be())
                }
            }

            fn from_bytes(bytes: Self::Bytes) -> Self {
                let integer = unsafe {
                    ::std::mem::transmute(bytes)
                };
                let unshifted = $type::from_be(integer);
                unshifted.wrapping_add($type::min_value())
            }
        }
    }
}

impl_signed_key!(i8, 1);
impl_signed_key!(i16, 2);
impl_signed_key!(i32, 4);
impl_signed_key!(i64, 8);
#[cfg(feature = "i128")]
impl_signed_key!(i128, 16);
#[cfg(target_pointer_width = "32")]
impl_signed_key!(isize, 4);
#[cfg(target_pointer_width = "64")]
impl_signed_key!(isize, 8);

#[cfg(test)]
mod test {
    use super::Key;

    /// Test that into_bytes and from_bytes reverse each other
    #[test]
    fn test_equivalence() {
        fn test_one<K: Key>(key: K) {
            let array = key.into_bytes();
            let reverse = K::from_bytes(array);
            assert_eq!(reverse, key);
        }
        // Unsigned
        test_one(10u8);
        test_one(10u16);
        test_one(10u32);
        test_one(10u64);
        #[cfg(feature = "i128")]
        test_one(10u128);

        // Signed
        test_one(-1i8);
        test_one(-1i16);
        test_one(-1i32);
        test_one(-1i64);
        #[cfg(feature = "i128")]
        test_one(-1i128);
    }
}
