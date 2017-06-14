use std::mem::size_of;
use std::cmp::Ordering;

pub fn partial_read(array: &[u8]) -> usize {
    debug_assert!(array.len() <= size_of::<usize>());
    array.iter()
        .fold(0, |acc, byte| (acc << 8) | *byte as usize)
}

pub fn partial_write(array: &mut [u8], mut value: usize) {
    for byte in array.iter_mut().rev() {
        *byte = (value & 0xff) as u8;
        value >>= 8;
    }
    debug_assert_eq!(value, 0, "Remaining value");
}

pub trait SliceExt {
    type Item;
    fn linear_search(&self, key: &Self::Item) -> Result<usize, usize>
        where Self::Item: Ord;
}

impl<T> SliceExt for [T] {
    type Item = T;
    // Linear search through a sorted slice
    fn linear_search(&self, key: &T) -> Result<usize, usize>
        where T: Ord {
        for (i, item) in self.iter().enumerate() {
            match item.cmp(key) {
                Ordering::Less => {},
                Ordering::Equal => {
                    return Ok(i);
                },
                Ordering::Greater => {
                    return Err(i);
                }
            }
        }
        Err(self.len())
    }
}

#[test]
fn test_partial_read() {
    let array = [1u8, 2u8];
    assert_eq!(partial_read(&array[..]), 0x102);
}

#[test]
fn test_partial_write() {
    let mut array = [0u8; 2];
    partial_write(&mut array[..], 0x102);
    assert_eq!(&array[..], &[0x1, 0x2][..]);
}

#[test]
fn test_read_write() {
    fn test_one(value: usize, size: usize) {
        #[cfg(target_pointer_width = "32")]
        let mut array = [0xffu8; 4];
        #[cfg(target_pointer_width = "64")]
        let mut array = [0xffu8; 8];
        partial_write(&mut array[..size], value);
        assert_eq!(partial_read(&array[..size]), value);
    }
    test_one(0x0201, 2);
    test_one(0x32659374, 4);
}

#[test]
fn test_find_item() {
    let array = [0, 1, 2, 3];
    assert_eq!(array.linear_search(&2), Ok(2));
}

#[test]
fn test_find_open() {
    let array = [0, 1, 1, 3];
    assert_eq!(array.linear_search(&2), Err(3));
}

#[test]
fn test_find_end_open() {
    let array = [0, 1, 2, 3];
    assert_eq!(array.linear_search(&4), Err(4))
}
