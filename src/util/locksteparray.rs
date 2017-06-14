use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::iter;

use nodrop::NoDrop;

use num_traits::{Unsigned, Zero, One};

pub struct OverflowError<T1, T2>(pub T1, pub T2);

#[derive(Debug)]
pub enum InsertError<T1, T2> {
    Overflow(T1, T2),
    OutOfBounds(T1, T2)
}

pub unsafe trait Array {
    type Item;
    type Index: Index;
    fn as_ptr(&self) -> *const Self::Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    fn capacity(&self) -> usize;
}

pub trait Index : Unsigned + ops::AddAssign + ops::SubAssign + Copy {
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
}

macro_rules! impl_index {
    ($index_type:ty) => (
        impl Index for $index_type {
            fn as_usize(self) -> usize {
                self as usize
            }

            fn from_usize(value: usize) -> Self {
                value as $index_type
            }
        }
    )
}

impl_index!(u8);
impl_index!(u16);
impl_index!(u32);
impl_index!(usize);

macro_rules! impl_array {
    ($index_type:ty, $len:expr ) => (
        unsafe impl<T> Array for [T; $len] {
            type Item = T;
            type Index = $index_type;
            #[inline(always)]
            fn as_ptr(&self) -> *const T {
                self as *const _ as *const _
            }

            #[inline(always)]
            fn as_mut_ptr(&mut self) -> *mut T {
                self as *mut _ as *mut _
            }

            #[inline(always)]
            fn capacity(&self) -> usize {
                $len
            }
        }
    )
}

macro_rules! impl_array_many {
    ($index_type:ty, $($len:expr),+) => (
        $(
            impl_array!($index_type, $len);
        )*
    );
}

impl_array_many!(u8, 1, 2, 7, 31, 256);

pub struct LockstepArray<A1: Array, A2: Array> {
    len: A1::Index,
    array1: NoDrop<A1>,
    array2: NoDrop<A2>
}

impl<A1, A2> LockstepArray<A1, A2> where A1: Array, A2: Array {
    pub fn new() -> LockstepArray<A1, A2> {
        LockstepArray {
            len: Zero::zero(),
            array1: NoDrop::new(unsafe {mem::uninitialized()}),
            array2: NoDrop::new(unsafe {mem::uninitialized()})
        }
    }

    pub fn from_arrays<B1, B2>(array1: B1, array2: B2) -> LockstepArray<A1, A2>
        where B1: Array<Item=A1::Item>, B2: Array<Item=A2::Item> {
        let mut lockstep = LockstepArray::<A1, A2>::new();
        // Eventually these will be encoded into the type system
        assert!(array1.capacity() <= lockstep.array1.capacity());
        assert!(array2.capacity() <= lockstep.array2.capacity());
        assert_eq!(array1.capacity(), array2.capacity());
        let len = A1::Index::from_usize(array1.capacity());
        unsafe {
            ptr::copy_nonoverlapping(array1.as_ptr(),
                                     lockstep.array1.as_mut_ptr(),
                                     array1.capacity());
            mem::forget(array1);
            ptr::copy_nonoverlapping(array2.as_ptr(),
                                     lockstep.array2.as_mut_ptr(),
                                     array2.capacity());
            mem::forget(array2);
        }
        lockstep.len = len;
        lockstep
    }

    pub fn push(&mut self, item1: A1::Item, item2: A2::Item)
                -> Result<(), OverflowError<A1::Item, A2::Item>> {
        if self.len.as_usize() == self.array1.capacity() {
            return Err(OverflowError(item1, item2));
        }
        unsafe {
            let p1 = self.array1.as_mut_ptr();
            ptr::write(p1.offset(self.len.as_usize() as isize), item1);
            let p2 = self.array2.as_mut_ptr();
            ptr::write(p2.offset(self.len.as_usize() as isize), item2);
        }
        self.len += One::one();
        Ok(())
    }

    pub fn pop(&mut self) -> Option<(A1::Item, A2::Item)> {
        if self.len == Zero::zero() {
            None
        } else {
            unsafe {
                // Calculate item 1's pointer
                let src1 = self.array1.as_mut_ptr()
                    .offset(self.len.as_usize() as isize);
                // Move item 1 out of the pointer
                let item1 = ptr::read(src1);
                // Calculate item 1's pointer
                let src2 = self.array2.as_mut_ptr()
                    .offset(self.len.as_usize() as isize);
                // Move item 1 out of the pointer
                let item2 = ptr::read(src2);
                // Update length
                self.len -= One::one();
                Some((item1, item2))
            }
        }
    }

    pub fn insert(&mut self, index: usize, item1: A1::Item, item2: A2::Item)
                  -> Result<(), InsertError<A1::Item, A2::Item>> {
        if index > self.len.as_usize() {
            Err(InsertError::OutOfBounds(item1, item2))
        } else if self.len.as_usize() == self.array1.capacity() {
            Err(InsertError::Overflow(item1, item2))
        } else {
            unsafe {
                // TODO: Tricky math, check indices and such
                let dest1 = self.array1.as_mut_ptr().offset(index as isize);
                ptr::copy(dest1, dest1.offset(1), self.len.as_usize() - index);
                ptr::write(dest1, item1);
                let dest2 = self.array2.as_mut_ptr().offset(index as isize);
                ptr::copy(dest2, dest2.offset(1), self.len.as_usize() - index);
                ptr::write(dest2, item2);
            }
            self.len += One::one();
            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<(A1::Item, A2::Item)> {
        if index >= self.len.as_usize() {
            None
        } else {
            let item1;
            let item2;
            unsafe {
                // Calculate item 1's pointer
                let src1 = self.array1.as_mut_ptr().offset(index as isize);
                // Move item 1 out of the pointer
                item1 = ptr::read(src1);
                // Shift down array 1 down by 1
                // Is self.len - index correct?
                ptr::copy(src1.offset(1), src1, self.len.as_usize() - index);
                // Calculate item 1's pointer
                let src2 = self.array2.as_mut_ptr().offset(index as isize);
                // Move item 1 out of the pointer
                item2 = ptr::read(src2);
                // Shift down array 1 down by 1
                // Is self.len - index correct?
                ptr::copy(src2, src2.offset(1), self.len.as_usize() - index);
                // Update length
                self.len -= One::one();
            }
            Some((item1, item2))
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len != Zero::zero()
    }

    pub fn len(&self) -> usize {
        return self.len.as_usize()
    }

    pub fn array1(&self) -> &[A1::Item] {
        unsafe {
            let ptr = self.array1.as_ptr();
            let len = self.len.as_usize();
            slice::from_raw_parts(ptr, len)
        }
    }

    pub fn array2(&self) -> &[A2::Item] {
        unsafe {
            let ptr = self.array2.as_ptr();
            let len = self.len.as_usize();
            slice::from_raw_parts(ptr, len)
        }
    }

    pub fn array1_mut(&mut self) -> &mut [A1::Item] {
        unsafe {
            let ptr = self.array1.as_mut_ptr();
            let len = self.len.as_usize();
            slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub fn array2_mut(&mut self) -> &mut [A2::Item] {
        unsafe {
            let ptr = self.array2.as_mut_ptr();
            let len = self.len.as_usize();
            slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub fn capacity(&self) -> usize {
        self.array1.capacity()
    }
}

impl<A1, A2> Default for LockstepArray<A1, A2> where A1: Array, A2: Array {
    fn default() -> Self {
        Self::new()
    }
}

impl<A1, A2> IntoIterator for LockstepArray<A1, A2> where A1: Array, A2: Array {
    type Item = (A1::Item, A2::Item);
    type IntoIter = IntoIter<A1, A2>;
    fn into_iter(self) -> Self::IntoIter {
        let LockstepArray { len, array1, array2 } = self;
        IntoIter {
            len: len.as_usize(),
            array1: array1,
            array2: array2,
            pos: 0
        }
    }
}

pub struct IntoIter<A1: Array, A2: Array> {
    len: usize,
    array1: NoDrop<A1>,
    array2: NoDrop<A2>,
    pos: usize
}

impl<A1: Array, A2: Array> Iterator for IntoIter<A1, A2> {
    type Item = (A1::Item, A2::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.len {
            None
        } else {
            unsafe {
                let ptr1 = self.array1.as_mut_ptr().offset(self.pos as isize);
                let item1 = ptr::read(ptr1);
                let ptr2 = self.array2.as_mut_ptr().offset(self.pos as isize);
                let item2 = ptr::read(ptr2);
                self.pos += 1;
                Some((item1, item2))
            }
        }
    }
}

#[test]
fn test_into_iter() {
    let mut locksteparray = LockstepArray::<[u8; 7], [u8; 7]>::new();
    locksteparray.insert(0, 4, 5).unwrap();
    locksteparray.insert(1, 3, 4).unwrap();
    let mut iter = locksteparray.into_iter();
    assert_eq!(iter.next(), Some((4, 5)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), None);
}
