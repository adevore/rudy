mod root_leaf;
mod rootptr;
mod jpm;
mod results;
mod iter;

use ::Key;
use self::rootptr::RootPtr;

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

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.root.remove(key)
    }

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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /*
    pub fn iter(&self) -> impl Iterator<Item=(K, &V)> {
        Iter::new(&self.root)
    }

    pub fn iter_mut(&self) -> impl Iterator<Item=(K, &mut V)> {
        IterMut::new(&mut self.root)
    }
    */

    pub fn memory_usage(&self) -> usize {
        self.root.memory_usage()
    }
}

impl<K: Key, V> Default for RudyMap<K, V> {
    fn default() -> RudyMap<K, V> {
        RudyMap::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_get_1() {
        let mut map = RudyMap::<usize, usize>::new();
        map.insert(4usize, 10usize);
        assert_eq!(map.get(4), Some(&10));
    }

    #[test]
    fn test_insert_get_2() {
        let mut map = RudyMap::<usize, usize>::new();
        map.insert(0usize, 10usize);
        map.insert(1usize, 20usize);
        assert_eq!(map.get(0), Some(&10));
        assert_eq!(map.get(1), Some(&20));
    }

    #[test]
    fn test_contains_key() {
        let mut map = RudyMap::<u32, u32>::new();
        assert_eq!(map.contains_key(0), false);
        assert_eq!(map.contains_key(1), false);

        assert_eq!(map.insert(0, 0), None);
        assert_eq!(map.contains_key(0), true);
        assert_eq!(map.contains_key(1), false);

        assert_eq!(map.remove(0), Some(0));
        assert_eq!(map.contains_key(0), false);
        assert_eq!(map.contains_key(1), false);
    }

    #[test]
    fn test_len() {
        let mut map = RudyMap::<u32, u32>::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);

        assert_eq!(map.insert(0, 0), None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.is_empty(), false);

        assert_eq!(map.insert(1, 1), None);
        assert_eq!(map.len(), 2);
        assert_eq!(map.is_empty(), false);

        assert_eq!(map.insert(0, 2), Some(0));
        assert_eq!(map.len(), 2);
        assert_eq!(map.is_empty(), false);

        assert_eq!(map.remove(0), Some(2));
        assert_eq!(map.len(), 1);
        assert_eq!(map.is_empty(), false);

        assert_eq!(map.remove(1), Some(1));
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_get_mut() {
        let mut map = RudyMap::<u32, u32>::new();
        assert!(map.get(0).is_none());
        assert!(map.get_mut(0).is_none());

        assert_eq!(map.insert(0, 0), None);
        assert_eq!(map.get(0), Some(&0));
        assert_eq!(map.get_mut(0), Some(&mut 0));
        assert_eq!(map.get(1), None);
        assert_eq!(map.get_mut(1), None);

        *map.get_mut(0).unwrap() += 42;
        assert_eq!(map.get(0), Some(&42));
        assert_eq!(map.get_mut(0), Some(&mut 42));

        map.remove(0);
    }
}
