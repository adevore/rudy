/// # Plans
/// Current this uses raw bit operations to provide a packed representation
/// with an interface similar to [Option<T>; N]. Once integer generics land,
/// it can instead use an abstraction layer to provide operations. Hopefully
/// that can be maintained as a separate crate.

use std::marker::PhantomData;
use std::mem;
use std::ptr;
use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::rudymap::results::{InsertResult, RemoveResult};
use ::Key;
use nodrop::NoDrop;

pub struct LeafBitmap<K: Key, V> {
    keys: [u8; 256 / 8],
    values: NoDrop<[V; 256]>,
    pd: PhantomData<K>
}

impl<K: Key, V> LeafBitmap<K, V> {
    pub fn new() -> LeafBitmap<K, V> {
        LeafBitmap {
            keys: [0; 256 / 8],
            values: unsafe { mem::zeroed() },
            pd: PhantomData
        }
    }
}

impl<K: Key, V> Drop for LeafBitmap<K, V> {
    fn drop(&mut self) {
        for index in 0..256 {
            let occupied = self.keys[index / 8] & (1 << (index % 8));
            if occupied != 0 {
                let mut value = &mut self.values[index];
                unsafe {
                    ptr::drop_in_place(value as *mut V);
                }
            }
        }
    }
}

#[derive(Debug)]
enum Place {
    Occupied(usize),
    Empty(usize)
}

fn singleton_index(key: &[u8], keys: &[u8; 256 / 8]) -> Place {
    debug_assert_eq!(key.len(), 1);
    let index = key[0] as usize;
    let occupied = keys[index / 8] & (1 << (index % 8));
    if occupied != 0 {
        Place::Occupied(index)
    } else {
        Place::Empty(index)
    }
}

impl<K: Key, V> JpmNode<K, V> for LeafBitmap<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        match singleton_index(key, &self.keys) {
            Place::Occupied(index) => Some(&self.values[index]),
            Place::Empty(index) => None
        }
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        match singleton_index(key, &self.keys) {
            Place::Occupied(index) => Some(&mut self.values[index]),
            Place::Empty(index) => None
        }
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        match singleton_index(key, &self.keys) {
            Place::Occupied(index) => {
                let place = &mut self.values[index];
                InsertResult::replace(place, value)
            },
            Place::Empty(index) => {
                let place = &mut self.values[index];
                self.keys[index / 8] |= 1 << (index % 8);
                unsafe {
                    ptr::write(place, value);
                }
                InsertResult::Success(None)
            }
        }
    }

    fn expand(self, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unreachable!();
    }

    fn remove(&mut self, key: &[u8]) -> RemoveResult<V> {
        match singleton_index(key, &self.keys) {
            Place::Occupied(index) => {
                let place = &mut self.values[index];
                let value = unsafe { ptr::read(place) };
                self.keys[index / 8] &= !(1 << (index % 8));
                RemoveResult::Success(Some(value))
            },
            Place::Empty(_) => {
                RemoveResult::Success(None)
            }
        }
    }

    fn shrink_remove(self, pop: usize, key: &[u8]) -> (InnerPtr<K, V>, V) {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::atomic::{AtomicUsize,Ordering};
    use util::test::Droppable;

    #[test]
    fn test_drop() {
        let drop_count = AtomicUsize::new(0);

        {
            // insert a single key
            let mut lb: LeafBitmap<u32, Droppable> = LeafBitmap::new();
            lb.insert(&[0], Droppable(&drop_count)).success();

            // inserting into an empty map should cause no drops
            assert_eq!(drop_count.load(Ordering::Acquire), 0);

            // overwriting the key should drop the old key
            lb.insert(&[0], Droppable(&drop_count)).success();
            assert_eq!(drop_count.load(Ordering::Acquire), 1);
        }

        // dropping the LeafBitmap should drop its only value
        assert_eq!(drop_count.load(Ordering::Acquire), 2);

        // reset the counter
        drop_count.store(0, Ordering::Release);

        {
            let mut lb: LeafBitmap<u32, Droppable> = LeafBitmap::new();
            for i in 0..256 {
                lb.insert(&[i as u8], Droppable(&drop_count)).success();
            }

            // inserting a full bitmap should cause no drops
            assert_eq!(drop_count.load(Ordering::Acquire), 0);

            // getting/get_muting should cause no drops
            lb.get(&[1]);
            lb.get_mut(&[2]);
            assert_eq!(drop_count.load(Ordering::Acquire), 0);

            // removing one key should cause one drop
            lb.remove(&[42]).success();
            assert_eq!(drop_count.load(Ordering::Acquire), 1);
        }

        // discarding the LeafBitmap should drop the others
        assert_eq!(drop_count.load(Ordering::Acquire), 256);
    }
}