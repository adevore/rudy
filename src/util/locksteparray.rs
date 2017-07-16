use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::iter;

use nodrop::NoDrop;

use num_traits::{Unsigned, Zero, One};

#[derive(Debug)]
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
    fn capacity() -> usize;
}

pub trait Index : Unsigned + ops::AddAssign + ops::SubAssign + Copy {
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
}

macro_rules! impl_index {
    ($index_type:ty) => (
        impl Index for $index_type {
            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }

            #[inline(always)]
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
    ($index_type:ty => $($len:expr),+ $(,)*) => (
        $(
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
                fn capacity() -> usize {
                    $len
                }
            }
        )*
    )
}

impl_array!(u8 => 1, 2, 7, 31, 256);

pub struct LockstepArray<A1: Array, A2: Array> {
    len: A1::Index,
    array1: NoDrop<A1>,
    array2: NoDrop<A2>
}

impl<A1, A2> LockstepArray<A1, A2> where A1: Array, A2: Array {
    pub fn new() -> LockstepArray<A1, A2> {
        // TODO: Eventually these will be encoded into the type system
        assert_eq!(A1::capacity(), A2::capacity());
        LockstepArray {
            len: Zero::zero(),
            array1: NoDrop::new(unsafe {mem::uninitialized()}),
            array2: NoDrop::new(unsafe {mem::uninitialized()})
        }
    }

    pub fn from_arrays<B1, B2>(array1: B1, array2: B2) -> LockstepArray<A1, A2>
        where B1: Array<Item=A1::Item>, B2: Array<Item=A2::Item> {
        let mut lockstep = LockstepArray::<A1, A2>::new();
        // TODO: Eventually these will be encoded into the type system
        assert!(B1::capacity() <= A1::capacity());
        assert!(B2::capacity() <= A2::capacity());
        assert_eq!(B1::capacity(), B2::capacity());
        let len = A1::Index::from_usize(B1::capacity());
        unsafe {
            ptr::copy_nonoverlapping(array1.as_ptr(),
                                     lockstep.array1.as_mut_ptr(),
                                     B1::capacity());
            mem::forget(array1);
            ptr::copy_nonoverlapping(array2.as_ptr(),
                                     lockstep.array2.as_mut_ptr(),
                                     B2::capacity());
            mem::forget(array2);
        }
        lockstep.len = len;
        lockstep
    }

    pub fn push(&mut self, item1: A1::Item, item2: A2::Item)
                -> Result<(), OverflowError<A1::Item, A2::Item>> {
        if self.len.as_usize() == A1::capacity() {
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
                    .offset(self.len.as_usize() as isize - 1);
                // Move item 1 out of the pointer
                let item1 = ptr::read(src1);
                // Calculate item 2's pointer
                let src2 = self.array2.as_mut_ptr()
                    .offset(self.len.as_usize() as isize - 1);
                // Move item 2 out of the pointer
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
        } else if self.len.as_usize() == A1::capacity() {
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
                ptr::copy(src1.offset(1), src1, self.len.as_usize() - index - 1);
                // Calculate item 2's pointer
                let src2 = self.array2.as_mut_ptr().offset(index as isize);
                // Move item 2 out of the pointer
                item2 = ptr::read(src2);
                // Shift down array 2 down by 1
                ptr::copy(src2.offset(1), src2, self.len.as_usize() - index - 1);
                // Update length
                self.len -= One::one();
            }
            Some((item1, item2))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == Zero::zero()
    }

    pub fn len(&self) -> usize {
        self.len.as_usize()
    }

    pub fn get(&self, index: usize) -> Option<(&A1::Item, &A2::Item)> {
        if index >= self.len.as_usize() {
            None
        } else {
            let item1 = unsafe {
                &*self.array1.as_ptr().offset(index as isize)
            };
            let item2 = unsafe {
                &*self.array2.as_ptr().offset(index as isize)
            };
            Some((item1, item2))
        }
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
        A1::capacity()
    }
}

impl<A1, A2> Drop for LockstepArray<A1, A2> where A1: Array, A2: Array {
    fn drop(&mut self) {
        for i in 0..self.len.as_usize() as isize {
            unsafe {
                ptr::drop_in_place(self.array1.as_mut_ptr().offset(i));
                ptr::drop_in_place(self.array2.as_mut_ptr().offset(i));
            }
        }
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
    fn into_iter(mut self) -> Self::IntoIter {
        // LockstepArray implements Drop, so we can't move out of it
        // We can, however, copy its arrays and zero its length, in which case drop() does nothing
        let mut array1: NoDrop<A1> = NoDrop::new(unsafe { mem::uninitialized() });
        let mut array2: NoDrop<A2> = NoDrop::new(unsafe { mem::uninitialized() });
        unsafe {
            ptr::copy_nonoverlapping(self.array1.as_ptr(),
                                     array1.as_mut_ptr(),
                                     A1::capacity());
            ptr::copy_nonoverlapping(self.array2.as_ptr(),
                                     array2.as_mut_ptr(),
                                     A2::capacity());
        }
        let mut len = A1::Index::from_usize(0);
        mem::swap(&mut len, &mut self.len);

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

impl<A1: Array, A2: Array> Drop for IntoIter<A1, A2> {
    fn drop(&mut self) {
        // drop any remaining items
        for i in self.pos..self.len {
            unsafe {
                ptr::drop_in_place(self.array1.as_mut_ptr().offset(i as isize));
                ptr::drop_in_place(self.array2.as_mut_ptr().offset(i as isize));
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

#[test]
fn test_remove() {
    let mut locksteparray = LockstepArray::<[u8; 7], [u8; 7]>::new();
    locksteparray.push(1, 2).unwrap();
    locksteparray.push(3, 4).unwrap();
    locksteparray.remove(0);
    assert_eq!(locksteparray.get(0), Some((&3, &4)));
    locksteparray.push(5, 6).unwrap();
    locksteparray.remove(1);
    assert_eq!(locksteparray.get(0), Some((&3, &4)));
}

#[test]
fn test_length() {
    let mut locksteparray = LockstepArray::<[u8; 7], [u8; 7]>::new();
    assert_eq!(locksteparray.len(), 0);
    assert!(locksteparray.is_empty());
    locksteparray.insert(0, 4, 5).unwrap();
    assert_eq!(locksteparray.len(), 1);
    locksteparray.insert(1, 3, 4).unwrap();
    assert_eq!(locksteparray.len(), 2);
    locksteparray.pop();
    assert_eq!(locksteparray.len(), 1);
    locksteparray.pop();
    assert_eq!(locksteparray.len(), 0);
    assert!(locksteparray.is_empty());
}

#[test]
fn test_drop() {
    use std::sync::atomic::{AtomicUsize,Ordering};
    use util::test::Droppable;

    let drop_count = AtomicUsize::new(0);

    {
        let mut locksteparray = LockstepArray::<[u8; 7], [Droppable; 7]>::new();
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // inserting should cause no drops
        locksteparray.insert(0, 255, Droppable(&drop_count)).unwrap();
        assert_eq!(locksteparray.len(), 1);
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // inserting to a new index should not cause drops
        locksteparray.insert(1, 255, Droppable(&drop_count)).unwrap();
        assert_eq!(locksteparray.len(), 2);
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // inserting to a used index should not cause a drop, since the array expands
        locksteparray.insert(0, 255, Droppable(&drop_count)).unwrap();
        assert_eq!(locksteparray.len(), 3);
        assert_eq!(drop_count.load(Ordering::Acquire), 0);
    }

    // dropping should cause all items to drop
    assert_eq!(drop_count.load(Ordering::Acquire), 3);

    // reset the counter
    drop_count.store(0, Ordering::Release);

    {
        let overflow_drop_count = AtomicUsize::new(0);
        let mut locksteparray = LockstepArray::<[u8; 7], [Droppable; 7]>::new();
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // insert seven things at location 0
        for i in 0..7 {
            locksteparray.insert(0, 255, Droppable(&drop_count)).unwrap();
            assert_eq!(locksteparray.len(), i+1);
            assert_eq!(drop_count.load(Ordering::Acquire), 0);
        }

        // inserting again should fail with an overflow
        match locksteparray.insert(0, 255, Droppable(&overflow_drop_count)) {
            Err(InsertError::Overflow(_i, _droppable)) => {
                // ...but no drops should yet occur
                assert_eq!(overflow_drop_count.load(Ordering::Acquire), 0);
                assert_eq!(drop_count.load(Ordering::Acquire), 0);
            }
            _ => {
                panic!("expected overflow")
            }
        }

        // overflowed element should now be dropped, since we dropped it when it went of scope
        assert_eq!(overflow_drop_count.load(Ordering::Acquire), 1);

        // array should still have seven with no interior elements being dropped
        assert_eq!(locksteparray.len(), 7);
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // pop one, let it drop
        locksteparray.pop().unwrap();
        assert_eq!(drop_count.load(Ordering::Acquire), 1);

        // remove another, let it drop
        locksteparray.remove(4).unwrap();
        assert_eq!(drop_count.load(Ordering::Acquire), 2);

        // drop the array
    }

    // all seven items should now be dropped
    assert_eq!(drop_count.load(Ordering::Acquire), 7);

    // reset the counter
    drop_count.store(0, Ordering::Release);
}

#[test]
fn test_into_iter_drop() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use util::test::Droppable;

    let drop_count = AtomicUsize::new(0);

    {
        // make an array with four items
        let mut locksteparray = LockstepArray::<[u8; 7], [Droppable; 7]>::new();
        for i in 0..4 {
            locksteparray.insert(0, 255, Droppable(&drop_count)).unwrap();
        }
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // convert to an iterator
        let mut into_iter = locksteparray.into_iter();
        assert_eq!(drop_count.load(Ordering::Acquire), 0);

        // grab an item, let it drop
        into_iter.next().unwrap();
        assert_eq!(drop_count.load(Ordering::Acquire), 1);
    }

    // ensure the unused items drop with the iter
    assert_eq!(drop_count.load(Ordering::Acquire), 4);
}

// TODO: Remove when this is detected at compile time.
#[test]
#[should_panic]
fn test_different_sized_arrays() {
    let _ = LockstepArray::<[u8; 2], [u8; 7]>::new();
}
