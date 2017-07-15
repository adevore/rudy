use ::Key;
use ::rudymap::RudyMap;

pub struct RudySet<K: Key> {
    map: RudyMap<K, ()>
}

impl<K: Key> RudySet<K> {
    pub fn new() -> RudySet<K> {
        RudySet {
            map: RudyMap::new()
        }
    }
    pub fn insert(&mut self, value: K) -> bool {
        self.map.insert(value, ()).is_some()
    }

    pub fn remove(&mut self, value: K) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn contains(&self, value: K) -> bool {
        self.map.contains_key(value)
    }

    pub fn clear(&mut self) {
        self.map = RudyMap::new();
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    /*
    pub fn iter(&self) -> Iter<'a, K> {
        Iter { iter: self.map.iter() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'a, K> {
        IterMut { iter: self.map.iter() }
    }
    */
}

/*
struct Iter<'a, K> {
    iter: rudymap::Iter<'a, K, ()>
}

impl<'a, K: Key> Iterator<K> for Iter<'a, K> {
    #[inline]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|(k, _)| k)
    }
}

struct IterMut<'a, K> {
    iter: rudymap::IterMut<'a, K, ()>
}

impl<'a, K: Key> Iterator<K> for IterMut<'a, K> {
    #[inline]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|(k, _)| k)
    }
}

impl<K: Key + PartialEq> PartialEq for RudySet<K> {
    fn eq(&self, other: &RudySet<K>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter())
            .all(|(a, b)| a == b)
    }
}

impl<K: Key + Eq> Eq for RudySet<K> {}

 */
