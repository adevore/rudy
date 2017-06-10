use ::Key;
use super::rootptr::RootPtr;

pub struct RudyMap<K: Key, V> {
    root: RootPtr<K, V>
}

impl<K: Key, V> RudyMap<K, V> {
    pub fn new() -> RudyMap<K, V> {
        RudyMap {
            root: RootPtr::empty()
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.root.insert(key, value)
    }

    /*
    fn remove(&mut self, key: K) -> Option<V> {
        self.root.remove(key)
    }
    */

    /*
    fn clear(&mut self) {
        self.root.clear()
    }
    */

    pub fn contains_key(&self, key: K) -> bool {
        // Because this does not visit the pointer returned by get(), there
        // will not be unnecessary cache fills
        self.get(key).is_some()
    }

    pub fn get(&self, key: K) -> Option<&V>{
        self.root.get(key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.root.get_mut(key)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    /*
    pub fn iter(&self) -> impl Iterator<Item=(K, &V)> {
        Iter::new(&self.root)
    }

    pub fn iter_mut(&self) -> impl Iterator<Item=(K, &mut V)> {
        IterMut::new(&mut self.root)
    }
    */
}
