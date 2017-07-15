use std::iter::FromIterator;
use std::mem;
use rudymap::root_leaf::RootLeaf;
use super::innerptr::InnerPtr;
use super::traits::JpmNode;
use ::rudymap::results::{InsertResult, RemoveResult};
use ::rudymap::rootptr::RootPtr;
use ::Key;

pub struct Jpm<K: Key, V> {
    head: InnerPtr<K, V>,
    len: usize
}

impl<K: Key, V> Jpm<K, V> {
    pub fn new() -> Jpm<K, V> {
        Jpm {
            head: InnerPtr::empty(),
            len: 0
        }
    }
}

impl<K: Key, V> RootLeaf<K, V> for Jpm<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        let bytes = key.into_bytes();
        self.head.get(bytes.as_ref())
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let bytes = key.into_bytes();
        self.head.get_mut(bytes.as_ref())
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        let bytes = key.into_bytes();
        InsertResult::Success(self.head.insert(bytes.as_ref(), value))
    }

    fn expand(mut self, key: K, value: V) -> RootPtr<K, V> {
        self.insert(key, value).success();
        Box::new(self).into()
    }

    fn remove(&mut self, key: K) -> RemoveResult<V> {
        let bytes = key.into_bytes();
        RemoveResult::Success(self.head.remove(bytes.as_ref()))
    }

    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V) {
        unimplemented!();
    }

    fn len(&self) -> usize {
        self.len
    }
    fn memory_usage(&self) -> usize {
        mem::size_of::<Self>() + self.head.memory_usage()
    }
}

impl<K: Key, V> FromIterator<(K, V)> for Jpm<K, V> {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=(K, V)> {
        let mut jpm = Jpm::new();
        for (key, value) in iter {
            jpm.insert(key, value).success();
        }
        jpm
    }
}
