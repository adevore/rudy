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
