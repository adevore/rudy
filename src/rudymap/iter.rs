use {Key};
use smallvec::SmallVec;
use super::rootptr::RootPtr;

macro_rules! iter_state {
    ($($type:ident),+) => {
        pub enum RootIterState<'a, K: Key, V> {
            $(
                $type($type<'a, K, V>),
            )*
        }

        $(
            impl<'a, K: Key, V> From<$type<'a, K, V>> for IterState<'a, K, V> {
                fn from(src: $type<'a, K, V>) -> Self {
                    IterState::$type(src)
                }
            }
        )*

        pub struct Iter<'a, K: Key, V> {
            stack: SmallVec<[IterState<'a, K, V>; 8]>
        }

        impl<'a, K: Key, V> Iter<'a, K, V> {
            pub fn new(root: &RootPtr<K, V>) -> Iter<K, V> {
                let mut stack = SmallVec::new();
                stack.push(root.iter_state().into());
                Iter { stack }
            }
        }

        impl<'a, K: Key, V: 'a> Iterator for Iter<'a, K, V> {
            type Item = (K, &'a V);
            fn next(&mut self) -> Option<(K, &'a V)> {
                match self.stack.pop() {
                    None => None,
                    $(
                        Some(IterState::$type(state)) => { None },
                    )*
                }
            }
        }
    }
}

iter_state!(
    EmptyState,
    Leaf1IterState,
    Leaf2IterState,
    VecIterState,
    JpmIterState
);
