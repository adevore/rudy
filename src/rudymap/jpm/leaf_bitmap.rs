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
