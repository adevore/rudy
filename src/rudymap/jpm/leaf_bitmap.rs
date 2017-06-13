use std::marker::PhantomData;
use std::mem;
use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::rudymap::results::InsertResult;
use ::Key;

pub struct LeafBitmap<K: Key, V> {
    array: [Option<V>; 256],
    pd: PhantomData<K>
}

impl<K: Key, V> LeafBitmap<K, V> {
    pub fn new() -> LeafBitmap<K, V> {
        LeafBitmap {
            array: unsafe { mem::zeroed() },
            pd: PhantomData
        }
    }
}

fn singleton_index(key: &[u8]) -> usize {
    debug_assert!(key.len() == 1);
    key[0] as usize
}

impl<K: Key, V> JpmNode<K, V> for LeafBitmap<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        self.array[singleton_index(key)].as_ref()
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        self.array[singleton_index(key)].as_mut()
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        let place = &mut self.array[singleton_index(key)];
        InsertResult::Success(mem::replace(place, Some(value)))
    }

    fn expand(self: Box<Self>, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unreachable!();
    }
}
