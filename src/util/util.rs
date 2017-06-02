use std::mem::size_of;

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
