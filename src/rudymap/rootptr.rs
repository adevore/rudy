use std::ptr;
use super::root_leaf::{RootLeaf, Empty, Leaf1, Leaf2, VecLeaf};
use super::jpm::Jpm;
use ::Key;
use std::marker::PhantomData;
use super::results::{InsertResult, RemoveResult};
use ::rudymap::iter::IterState;

fn into_raw<T>(node: Box<T>) -> *mut () {
    Box::into_raw(node) as *mut ()
}

unsafe fn from_raw<T>(ptr: *mut ()) -> Box<T> {
    Box::from_raw(ptr as *mut T)
}

macro_rules! impl_root_ptr {
    ($($type_code:expr => $type_name:ident),+) => {
        pub struct RootPtr<K: Key, V> {
            // TODO: If NonZero stabilizes, adapt this to be non-zero when empty
            word: usize,
            phantomdata: PhantomData<(K, V)>
        }

        pub enum RootRef<'a, K: Key + 'a, V: 'a> {
            Empty(Empty<K, V>),
            $(
                $type_name(&'a $type_name<K, V>),
            )*
        }

        pub enum RootMut<'a, K: Key + 'a, V: 'a> {
            Empty(Empty<K, V>),
            $(
                $type_name(&'a mut $type_name<K, V>),
            )*
        }

        pub enum RootOwned<K: Key, V> {
            Empty(Box<Empty<K, V>>),
            $(
                $type_name(Box<$type_name<K, V>>),
            )*
        }

        impl<K: Key, V> RootPtr<K, V> {
            unsafe fn new(ptr: *mut(), type_code: usize) -> RootPtr<K, V> {
                debug_assert_eq!(ptr as usize & 0b111, 0,
                              "Low bits of root ptr {:?} are set", ptr);
                RootPtr {
                    word: ptr as usize | type_code,
                    phantomdata: PhantomData
                }
            }

            pub fn empty() -> RootPtr<K, V> {
                unsafe {
                    Self::new(ptr::null_mut(), 0)
                }
            }

            pub fn as_ref(&self) -> RootRef<K, V> {
                match self.type_code() {
                    0 => RootRef::Empty(Empty::new()),
                    $(
                        $type_code => RootRef::$type_name(
                            unsafe { &*(self.ptr() as *const $type_name<K, V>) }
                        ),
                    )*
                    x => panic!("Unknown type code in root pointer: {}", x)
                }
            }

            pub fn as_mut(&mut self) -> RootMut<K, V> {
                match self.type_code() {
                    0 => RootMut::Empty(Empty::new()),
                    $(
                        $type_code => RootMut::$type_name(
                            unsafe { &mut *(self.ptr() as *mut $type_name<K, V>) }
                        ),
                    )*
                    x => panic!("Unknown type code in root pointer: {}", x)
                }
            }

            pub fn into_owned(self) -> RootOwned<K, V> {
                let ptr = self.ptr_mut();
                let type_code = self.type_code();
                ::std::mem::forget(self);
                match type_code {
                    0 => RootOwned::Empty(Box::new(Empty::new())),
                    $(
                        $type_code => RootOwned::$type_name(
                            unsafe {
                                Box::from_raw(ptr as *mut $type_name<K, V>)
                            }
                        ),
                    )*
                    x => panic!("Unknown type code in root pointer: {}", x)
                }
            }

            pub fn get(&self, key: K) -> Option<&V> {
                match self.as_ref() {
                    RootRef::Empty(_) => None,
                    $(
                        RootRef::$type_name(node) => node.get(key),
                    )*
                }
            }

            pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
                match self.as_mut() {
                    RootMut::Empty(_) => None,
                    $(
                        RootMut::$type_name(node) => node.get_mut(key),
                    )*
                }
            }

            fn type_code(&self) -> usize {
                self.word & 0b111
            }

            fn ptr(&self) -> *const () {
                (self.word & !0b111) as *const ()
            }

            fn ptr_mut(&self) -> *mut () {
                (self.word & !0b111) as *mut ()
            }
        }

        impl<K: Key, V> Drop for RootPtr<K, V> {
            fn drop(&mut self) {
                self.take().into_owned();
            }
        }

        $(
            impl<K: Key, V> From<Box<$type_name<K, V>>> for RootPtr<K, V> {
                fn from(src: Box<$type_name<K, V>>) -> RootPtr<K, V> {
                    let ptr = Box::into_raw(src);
                    unsafe {
                        RootPtr::new(ptr as *mut (), $type_code)
                    }
                }
            }
        )*

        impl_root_ptr_dispatch!(
            $($type_code => $type_name,)*
            0 => Empty
        );
    }
}

macro_rules! impl_root_ptr_dispatch {
    ($($type_code:expr => $type_name:ident),+) => {
        impl<K: Key, V> RootPtr<K, V> {
            pub fn len(&self) -> usize {
                match self.as_ref() {
                    $(
                        RootRef::$type_name(node) => node.len(),
                    )*
                }
            }

            pub fn insert(&mut self, key: K, value: V) -> Option<V> {
                let result = match self.as_mut() {
                    $(
                        RootMut::$type_name(mut node) => node.insert(key, value),
                    )*
                };
                match result {
                    InsertResult::Success(evicted) => {
                        evicted
                    },
                    InsertResult::Resize(value) => {
                        *self = self.take().expand(key, value);
                        None
                    }
                }
            }

            pub fn expand(self, key: K, value: V) -> RootPtr<K, V> {
                match self.into_owned() {
                    $(
                        RootOwned::$type_name(node) => {
                            node.expand(key, value)
                        },
                    )*
                }
            }

            pub fn remove(&mut self, key: K) -> Option<V> {
                let result = match self.as_mut() {
                    $(
                        RootMut::$type_name(mut node) => node.remove(key),
                    )*
                };
                match result {
                    RemoveResult::Success(evicted) => {
                        evicted
                    },
                    RemoveResult::Downsize => {
                        let (ptr, value) = self.take().shrink_remove(key);
                        *self = ptr;
                        Some(value)
                    }
                }
            }

            pub fn shrink_remove(self, key: K) -> (RootPtr<K, V>, V) {
                match self.into_owned() {
                    $(
                        RootOwned::$type_name(node) => {
                            node.shrink_remove(key)
                        },
                    )*
                }
            }

            pub fn iter_state(&self) -> IterState<K, V> {
                match self.as_ref() {
                    $(
                        RootRef::$type_name(node) => node.iter_state(),
                    )*
                }
            }

            pub fn take(&mut self) -> RootPtr<K, V> {
                ::std::mem::replace(self, RootPtr::empty())
            }
        }
    }
}

impl_root_ptr!(
    1 => Leaf1,
    2 => Leaf2,
    3 => VecLeaf,
    4 => Jpm
);
