use ::Key;

pub struct RudyMap<K: Key, V> {
    root: RootPtr<K, V>
}

impl RudyMap<K: Key, V> {
    fn new() -> RudyMap<K, V> {
        RudyMap {
            root: RootPtr::empty()
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.root.as_mut_ref().insert(key, value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        self.root.remove(key)
    }

    fn clear(&mut self) {
        self.root.clear()
    }

    fn contains_key(&self, key: K) -> bool {
        // Because this does not visit the pointer returned by get(), there
        // will not be unnecessary cache fills
        self.get(key).is_some()
    }

    fn get(&self, key: K) -> Option<&V>{
        self.get(key)
    }

    fn get_mut(&self, key: K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn iter(&self) -> Iter {
        Iter::new(self.root)
    }

    fn iter_mut(&self) -> IterMut {
        IterMut::new(self.root)
    }
}

