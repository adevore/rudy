use super::empty::Empty;
use super::branch_linear::BranchLinear;
use super::branch_bitmap::BranchBitmap;
use super::branch_uncompressed::BranchUncompressed;
use super::leaf_linear::LeafLinear;
use super::leaf_bitmap::LeafBitmap;
use super::traits::JpmNode;
use ::rudymap::results::{InsertResult, RemoveResult};
use ::util::{partial_write, partial_read};
use ::Key;
use std::mem;

#[cfg(target_pointer_width = "32")]
pub struct Population {
    inner: [u8; 3]
}

#[cfg(target_pointer_width = "64")]
pub struct Population {
    inner: [u8; 7]
}

impl Population {
    fn new(value: usize) -> Population {
        let mut pop = Population {
            inner: Default::default()
        };
        partial_write(&mut pop.inner, value);
        pop
    }

    fn as_usize(&self) -> usize {
        partial_read(&self.inner)
    }
}

macro_rules! make_inner_ptr {
    ($($type:ident),+) => {
        pub enum Ref<'a, K: Key + 'a, V: 'a> {
            $(
                $type(&'a $type<K, V>),
            )*
        }

        pub enum Mut<'a, K: Key + 'a, V: 'a> {
            $(
                $type(&'a mut $type<K, V>),
            )*
        }

        pub enum InnerPtr<K: Key, V> {
            $(
                $type(Box<$type<K, V>>, Population),
            )*
        }

        impl<K: Key, V> InnerPtr<K, V> {
            pub fn new<B: IntoPtr<K, V>>(boxed: Box<B>, pop: usize) -> InnerPtr<K, V> {
                IntoPtr::into_ptr(boxed, pop)
            }

            pub fn as_ref(&self) -> Ref<K, V> {
                match *self {
                    $(
                        InnerPtr::$type(ref target, ..) => {
                            Ref::$type(target)
                        },
                    )*
                }
            }

            pub fn as_mut(&mut self) -> Mut<K, V> {
                match *self {
                    $(
                        InnerPtr::$type(ref mut target, ..) => {
                            Mut::$type(target)},
                    )*
                }
            }

            pub fn get(&self, bytes: &[u8]) -> Option<&V> {
                match self.as_ref() {
                    $(
                        Ref::$type(target) => {
                            target.get(bytes)
                        },
                    )*
                }
            }

            pub fn get_mut(&mut self, bytes: &[u8]) -> Option<&mut V> {
                match self.as_mut() {
                    $(
                        Mut::$type(target) => {
                            target.get_mut(bytes)
                        },
                    )*
                }
            }

            pub fn insert(&mut self, key: &[u8], value: V) -> Option<V> {
                let insert_result = match self.as_mut() {
                    $(
                        Mut::$type(target) => {
                            target.insert(key, value)
                        },
                    )*
                };
                match insert_result {
                    InsertResult::Success(evicted) => evicted,
                    InsertResult::Resize(value) => {
                        *self = self.take().expand(key, value);
                        None
                    }
                }
            }

            fn expand(self, key: &[u8], value: V) -> InnerPtr<K, V> {
                match self {
                    $(
                        InnerPtr::$type(target, pop) => {
                            let new_pop = pop.as_usize() + 1;
                            target.expand(new_pop, key, value)
                        },
                    )*
                }
            }

            pub fn remove(&mut self, key: &[u8]) -> Option<V> {
                let remove_result = match self.as_mut() {
                    $(
                        Mut::$type(target) => {
                            target.remove(key)
                        },
                    )*
                };
                match remove_result {
                    RemoveResult::Success(evicted) => evicted,
                    RemoveResult::Downsize => {
                        let (ptr, value) = self.take().shrink_remove(key);
                        *self = ptr;
                        Some(value)
                    }
                }
            }

            fn shrink_remove(self, key: &[u8]) -> (InnerPtr<K, V>, V) {
                match self {
                    $(
                        InnerPtr::$type(target, pop) => {
                            let new_pop = pop.as_usize() - 1;
                            target.shrink_remove(new_pop, key)
                        },
                    )*
                }
            }

            pub fn take(&mut self) -> InnerPtr<K, V> {
                ::std::mem::replace(self, InnerPtr::default())
            }

            // In general, an `InnerPtr` or array of `InnerPtr`s will be included in some other data
            // structure which already counts its own size in a `memory_usage()` function. Thus, it
            // is not useful for `InnerPtr` to report its own memory consumption.
            //
            // However, it _is_ useful to ask an `InnerPtr` about the memory consumption of its
            // target without including the `InnerPtr` itself. This is what `target_memory_usage()`
            // does.
            pub fn target_memory_usage(&self) -> usize {
                match *self {
                    $(
                        InnerPtr::$type(ref target, ref _pop) => {
                            target.memory_usage()
                        },
                    )*
                }
            }
        }


        pub trait IntoPtr<K: Key, V> {
            fn into_ptr(from: Box<Self>, pop: usize) -> InnerPtr<K, V>;
        }

        $(
            impl<K: Key, V> IntoPtr<K, V> for $type<K, V> {
                fn into_ptr(from: Box<Self>, pop: usize) -> InnerPtr<K, V> {
                    InnerPtr::$type(from, Population::new(pop))
                }
            }
        )*
    }
}

make_inner_ptr!(Empty,
                BranchLinear, BranchBitmap, BranchUncompressed,
                LeafLinear, LeafBitmap);

impl<K: Key, V> InnerPtr<K, V> {
    pub fn empty() -> InnerPtr<K, V> {
        // Empty is a ZST, so this does not actually allocate
        InnerPtr::new(Box::new(Empty::new()), 0)
    }
}

impl<K: Key, V> Default for InnerPtr<K, V> {
    fn default() -> InnerPtr<K, V> {
        InnerPtr::empty()
    }
}

#[cfg(test)]
mod test {
    use super::InnerPtr;
    use super::BranchLinear;
    use super::BranchBitmap;
    use super::BranchUncompressed;
    use super::LeafLinear;
    use super::LeafBitmap;

    #[test]
    fn test_new() {
        type WordInnerPtr = InnerPtr<usize, usize>;
        WordInnerPtr::empty();
        WordInnerPtr::new(Box::new(BranchLinear::new()), 0);
        WordInnerPtr::new(Box::new(BranchBitmap::new()), 0);
        WordInnerPtr::new(Box::new(BranchUncompressed::new()), 0);
        //WordInnerPtr::new(Box::new(LeafLinear::new()), 0);
        //WordInnerPtr::new(Box::new(LeafBitmap::new()), 0);
    }
}
