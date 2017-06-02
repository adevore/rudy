use util::locksteparray::{LockstepArray, InsertError};
use super::jpm::jpm_root::JPM;
use ::Key;
use ::rudymap::results::InsertResult;

pub trait RootLeaf<K, V> {
    type OverflowNode: RootLeaf<K, V>;
    fn get(&self, key: K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> InsertResult<V>;
    fn expand(self, key: K, value: V) -> Self::OverflowNode;
    fn len(&self) -> usize;
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

/// A leaf root with one item.
impl<K: Key, V> RootLeaf<K, V> for Leaf1<K, V> {
    type OverflowNode = Leaf2<K, V>;
    fn get(&self, key: K) -> Option<&V> {
        if self.key == key {
            Some(&self.value)
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        if self.key == key {
            self.value = value;
            InsertResult::Success
        } else {
            InsertResult::Resize(value)
        }
    }

    fn expand(self, key: K, value: V) -> Leaf2<K, V> {
        Leaf2::new(self.key, self.value, key, value)
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
    type OverflowNode = VecLeaf<K, V>;
    fn get(&self, key: K) -> Option<&V> {
        self.keys.iter()
            .zip(self.values.iter())
            .find(|&(&leaf_key, _)| leaf_key == key)
            .map(|(key, value)| value)
    }
    /// Attempt to insert, fail if we didn't find a key to replace
    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        for (i, leaf_key) in self.keys.iter().enumerate() {
            if key == *leaf_key {
                self.values[i] = value;
                return InsertResult::Success
            }
        }
        InsertResult::Resize(value)
    }

    fn expand(self, key: K, value: V) -> VecLeaf<K, V> {
        let Leaf2 { keys, values } = self;
        let mut leaf = VecLeaf::from_arrays(keys, values);
        leaf.insert(key, value).success();
        leaf
    }

    fn len(&self) -> usize {
        2
    }
}

pub struct VecLeaf<K: Key, V> {
    array: LockstepArray<[K; 31], [V; 31]>
}

impl<K: Key, V> VecLeaf<K, V> {
    fn new() -> VecLeaf<K, V> {
        // TODO Copy memory from values
        VecLeaf {
            array: LockstepArray::new()
        }
    }

    fn from_arrays(keys: [K; 2], values: [V; 2]) -> VecLeaf<K, V> {
        /*
        VecLeaf {
            array: LockstepArray::from_arrays(keys, values)
        }
         */
        unimplemented!();
    }
}

impl<K: Key, V> RootLeaf<K, V> for VecLeaf<K, V> {
    type OverflowNode = JPM<K, V>;
    fn get(&self, key: K) -> Option<&V> {
        self.array.array1()
            .binary_search(&key)
            .ok()
            .map(|index| &self.array.array2()[index])
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        match self.array.array1().binary_search(&key) {
            Ok(replace) => {
                self.array.array2_mut()[replace] = value;
                InsertResult::Success
            },
            Err(insert) => match self.array.insert(insert, key, value) {
                Ok(()) => InsertResult::Success,
                Err(InsertError::Overflow(key, value)) => {
                    InsertResult::Resize(value)
                },
                Err(InsertError::OutOfBounds(..)) => {
                    unreachable!()
                }
            }
        }
    }

    fn expand(self, key: K, value: V) -> JPM<K, V> {
        // TODO: Implement transition
        JPM::new()
    }

    fn len(&self) -> usize {
        self.array.len()
    }
}

