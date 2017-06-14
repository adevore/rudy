use ::Key;
use super::innerptr::{InnerPtr, IntoPtr};
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use super::branch_uncompressed::BranchUncompressed;
use std::iter::FromIterator;

struct Subexpanse<K: Key, V> {
    pub bitmap: u32,
    pub ptr: Option<Box<[InnerPtr<K, V>; 32]>>
}

impl<K: Key, V> Default for Subexpanse<K, V> {
    fn default() -> Subexpanse<K, V> {
        Subexpanse {
            bitmap: 0,
            ptr: None
        }
    }
}

impl<K: Key, V> Subexpanse<K, V> {
    fn is_set(&self, sub_byte: u8) -> bool {
        self.bitmap & (1 << sub_byte as u32) != 0
    }

    pub fn get(&self, sub_byte: u8, subkey: &[u8]) -> Option<&V> {
        if self.is_set(sub_byte) {
            self.ptr.as_ref()
                .unwrap()[sub_byte as usize]
                .get(subkey)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, sub_byte: u8, subkey: &[u8]) -> Option<&mut V> {
        if self.is_set(sub_byte) {
            self.ptr.as_mut()
                .unwrap()[sub_byte as usize]
                .get_mut(subkey)
        } else {
            None
        }
    }

    pub fn insert(&mut self, sub_byte: u8, subkey: &[u8], value: V) -> InsertResult<V> {
        if self.ptr.is_none() {
            self.ptr = Some(Default::default());
        }
        let evicted = self.ptr.as_mut()
            .unwrap()[sub_byte as usize]
            .insert(subkey, value);
        self.bitmap |= 1 << sub_byte as u32;
        InsertResult::Success(evicted)
    }

    pub fn insert_ptr(&mut self, sub_byte: u8, ptr: InnerPtr<K, V>) {
        if self.ptr.is_none() {
            self.ptr = Some(Default::default());
        }
        self.ptr.as_mut().unwrap()[sub_byte as usize] = ptr;
        self.bitmap |= 1 << sub_byte as u32;
    }
}

pub struct BranchBitmap<K: Key, V> {
    subexpanses: [Subexpanse<K, V>; 8]
}

impl<K: Key, V> BranchBitmap<K, V> {
    pub fn new() -> BranchBitmap<K, V> {
        BranchBitmap {
            subexpanses: Default::default()
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for BranchBitmap<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        let (&byte, subkey) = key.split_first().unwrap();
        self.subexpanses[byte as usize / 32].get(byte % 32, subkey)
    }
    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        let (&byte, subkey) = key.split_first().unwrap();
        self.subexpanses[byte as usize / 32].get_mut(byte % 32, subkey)
    }
    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        let (&byte, subkey) = key.split_first().unwrap();
        self.subexpanses[byte as usize / 32].insert(byte % 32, subkey, value)
    }
    fn expand(self, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unreachable!()
    }
}

impl<K: Key, V> FromIterator<(u8, InnerPtr<K, V>)> for BranchBitmap<K, V> {
    fn from_iter<I>(iter: I) -> BranchBitmap<K, V>
        where I: IntoIterator<Item=(u8, InnerPtr<K, V>)> {
        let mut node = BranchBitmap::new();
        for (k, v) in iter {
            node.subexpanses[k as usize / 32].insert_ptr(k % 32, v);
        }
        node
    }
}
