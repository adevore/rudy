enum IterState {
    
}

pub struct Iter<'a, K: Key, V> {
    stack: ArrayVec<K::IterStack<'a>>
}

impl<'a, K: Key, V> Iter {
    pub fn new(root: RootPtr) -> Iter {
        let mut stack = ArrayVec::new();
        stack.insert()
    }
}
