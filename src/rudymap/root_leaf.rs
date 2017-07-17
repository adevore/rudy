use std::marker::PhantomData;
use std::mem;
use std::ptr;
use util::locksteparray;
use util::SliceExt;
use super::jpm::jpm_root::Jpm;
use ::Key;
use ::rudymap::results::{InsertResult, RemoveResult};
use std::iter;
use super::rootptr::RootPtr;

pub trait RootLeaf<K: Key, V> {
    fn get(&self, key: K) -> Option<&V>;
    fn get_mut(&mut self, key: K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V) -> InsertResult<V>;
    fn expand(self, key: K, value: V) -> RootPtr<K, V>;
    fn remove(&mut self, key: K) -> RemoveResult<V>;
    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V);
    fn len(&self) -> usize;
}

pub struct Empty<K: Key, V>(PhantomData<(K, V)>);

impl<K: Key, V> Empty<K, V> {
    pub fn new() -> Empty<K, V> {
        Empty(PhantomData)
    }
}

impl<K: Key, V> RootLeaf<K, V> for Empty<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        None
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        None
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        InsertResult::Resize(value)
    }

    fn remove(&mut self, key: K) -> RemoveResult<V> {
        RemoveResult::Success(None)
    }

    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V){
        unreachable!();
    }

    fn expand(self, key: K, value: V) -> RootPtr<K, V> {
        Box::new(Leaf1::new(key, value)).into()
    }

    fn len(&self) -> usize {
        0
    }
}

impl<K: Key, V> Default for Empty<K, V> {
    fn default() -> Empty<K, V> {
        Empty::new()
    }
}

impl<'a, K: Key + 'a, V: 'a> IntoIterator for &'a Empty<K, V> {
    type Item = (K, &'a V);
    type IntoIter = iter::Empty<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}

pub struct Leaf1<K: Key, V> {
    key: K,
    value: V
}

impl<K: Key, V> Leaf1<K, V> {
    pub fn new(key: K, value: V) -> Leaf1<K, V> {
        Leaf1 { key, value }
    }
}

impl<'a, K: Key + 'a, V: 'a> IntoIterator for &'a Leaf1<K, V> {
    type Item = (K, &'a V);
    type IntoIter = iter::Once<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        iter::once((self.key, &self.value))
    }
}

/// A leaf root with one item.
impl<K: Key, V> RootLeaf<K, V> for Leaf1<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        if self.key == key {
            Some(&self.value)
        } else {
            None
        }
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if self.key == key {
            Some(&mut self.value)
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        if self.key == key {
            InsertResult::replace(&mut self.value, value)
        } else {
            InsertResult::Resize(value)
        }
    }

    fn expand(self, key: K, value: V) -> RootPtr<K, V> {
        Box::new(Leaf2::new(self.key, self.value, key, value)).into()
    }

    fn remove(&mut self, key: K) -> RemoveResult<V> {
        if self.key == key {
            RemoveResult::Downsize
        } else {
            RemoveResult::Success(None)
        }
    }

    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V) {
        let Leaf1 { key: node_key, value } = self;
        let ptr = RootPtr::empty();
        debug_assert_eq!(node_key, key);
        (ptr, value)
    }

    fn len(&self) -> usize {
        1
    }
}

pub struct Leaf2<K: Key, V> {
    keys: [K; 2],
    values: [V; 2]
}

impl<K: Key, V> Leaf2<K, V> {
    pub fn new(key1: K, value1: V, key2: K, value2: V) -> Leaf2<K, V> {
        if key1 < key2 {
            Leaf2 {
                keys: [key1, key2],
                values: [value1, value2]
            }
        } else {
            Leaf2 {
                keys: [key2, key1],
                values: [value2, value1]
            }
        }
    }
}

impl<K: Key, V> RootLeaf<K, V> for Leaf2<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        self.keys.iter()
            .zip(self.values.iter())
            .find(|&(&leaf_key, _)| leaf_key == key)
            .map(|(key, value)| value)
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.keys.iter()
            .zip(self.values.iter_mut())
            .find(|&(&leaf_key, _)| leaf_key == key)
            .map(|(key, value)| value)
    }

    /// Attempt to insert, fail if we didn't find a key to replace
    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        for (i, leaf_key) in self.keys.iter().enumerate() {
            if key == *leaf_key {
                return InsertResult::replace(&mut self.values[i], value);
            }
        }
        InsertResult::Resize(value)
    }

    fn expand(self, key: K, value: V) -> RootPtr<K, V> {
        let Leaf2 { keys, values } = self;
        let mut leaf = Box::new(VecLeaf::from_arrays(keys, values));
        leaf.insert(key, value).success();
        leaf.into()
    }

    fn remove(&mut self, key: K) -> RemoveResult<V> {
        self.keys.iter()
            .find(|&&k| k == key)
            .map(|_| RemoveResult::Downsize)
            .unwrap_or(RemoveResult::Success(None))
    }

    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V) {
        let Leaf2 { keys, mut values } = self;
        let key1 = keys[0];
        let key2 = keys[1];
        let (value1, value2);
        unsafe {
            value1 = ptr::read(&mut values[0]);
            value2 = ptr::read(&mut values[1]);
            mem::forget(values);
        }
        if key1 == key {
            let ptr = Box::new(Leaf1::new(key2, value2)).into();
            (ptr, value1)
        } else {
            let ptr = Box::new(Leaf1::new(key1, value1)).into();
            (ptr, value2)
        }
    }

    fn len(&self) -> usize {
        2
    }
}

pub struct VecLeaf<K: Key, V> {
    array: locksteparray::LockstepArray<[K; 31], [V; 31]>
}

impl<K: Key, V> VecLeaf<K, V> {
    fn new() -> VecLeaf<K, V> {
        // TODO Copy memory from values
        VecLeaf {
            array: locksteparray::LockstepArray::new()
        }
    }

    fn from_arrays(keys: [K; 2], values: [V; 2]) -> VecLeaf<K, V> {
        VecLeaf {
            array: locksteparray::LockstepArray::from_arrays(keys, values)
        }
    }
}

impl<K: Key, V> IntoIterator for VecLeaf<K, V> {
    type Item = (K, V);
    type IntoIter = locksteparray::IntoIter<[K; 31], [V; 31]>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}


impl<K: Key, V> RootLeaf<K, V> for VecLeaf<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        self.array.array1()
            .iter()
            .position(|&k| k == key)
            .map(|index| &self.array.array2()[index])
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.array.array1_mut()
            .iter()
            .position(|&k| k == key)
            .map(move |index| &mut self.array.array2_mut()[index])
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        match self.array.array1().linear_search(&key) {
            Ok(replace) => {
                InsertResult::replace(&mut self.array.array2_mut()[replace],
                                      value)
            },
            Err(insert) => match self.array.insert(insert, key, value) {
                Ok(()) => InsertResult::Success(None),
                Err(locksteparray::InsertError::Overflow(key, value)) => {
                    InsertResult::Resize(value)
                },
                Err(locksteparray::InsertError::OutOfBounds(..)) => {
                    unreachable!()
                }
            }
        }
    }

    fn expand(self, key: K, value: V) -> RootPtr<K, V> {
        let mut jpm: Jpm<K, V> = self.into_iter().collect();
        jpm.insert(key, value).success();
        Box::new(jpm).into()
    }

    fn remove(&mut self, key: K) -> RemoveResult<V> {
        let evicted = self.array.array1_mut()
            .iter()
            .position(|&k| k == key)
            .and_then(|index| self.array.remove(index))
            .map(|(_, value)| value);
        RemoveResult::Success(evicted)
    }

    fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V) {
        unreachable!()
    }

    fn len(&self) -> usize {
        self.array.len()
    }
}
