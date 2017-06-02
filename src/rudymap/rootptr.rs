use std::ptr;
use root_leaf::{Leaf1, Leaf2, VecLeaf};

macro_rules! impl_root_ptr() {
    ($($type_code:expr => $type_name:ident)+) => {
        struct RootPtr<K: Key, V> {
            // TODO: If NonZero stabilizes, adapt this to be non-zero when empty
            word: usize
        }

        impl<K: Key, V> RootPtr<K, V> {
            pub fn empty() -> RootPtr<K, V> {
                unsafe { RootPtr::new(ptr::null_mut(), 0) }
            }

            unsafe fn new(ptr: *mut (), type_code: usize) -> RootPtr<K, V> {
                RootPtr { word: ptr as usize | type_code }
            }

            pub fn as_ref(&self) -> RootRef<K, V> {
                match self.type_code() {
                    0 => RootRef::Empty,
                    $(
                        $type_code => RootRef::$type_name(
                            unsafe { $type_name::as_ref(self.ptr()) }
                        ),
                    )*
                    x => panic!("Unknown type code in root pointer: {}", x)
                }
            }
        }

        pub fn as_mut(&mut self) -> RootMut<K, V> {
            match self.type_code() {
                0 => RootMut::Empty,
                $(
                    $type_code => RootMut::$type_name(
                        unsafe { $type_name::as_mut(self.ptr_mut()) }
                    ),
                )*
                x => panic!("Unknown type code in root pointer: {}", x)
            }
        }

        pub fn len(&self) -> usize {
            match self.as_mut() {
                RootRef::Empty => 0,
                $(
                    RootRef::$type_name(node) => node.len()
                )*
            }
        }

        fn type_code(&self) -> usize {
            word & 0b111
        }

        fn ptr(&self) -> *const () {
            (word & ~0b111) as *const ()
        }

        fn ptr_mut(&self) -> *mut () {
            (word & ~0b111) as *mut ()
        }
    }
}

impl Drop for RootPtr<K, V> {
    fn drop(&mut self) {
        unsafe {
            match self.type_code() {
                0 => {},
                1 => {
                    Leaf1::into_box(self.ptr_mut());
                },
                2 => {
                    Leaf2::into_box(self.ptr_mut());
                },
                3 => {
                    VecLeaf::into_box(self.ptr_mut());
                },
                4 => {
                    JPM::into_box(self.ptr_mut());
                }
            }
        }
    }
}

      
    }
}


trait RootNode {
    fn type_code() -> usize;

    fn into_raw(node: Box<Self>) -> *mut () {
        node.into_raw() as *mut ()
    }

    unsafe fn from_raw(ptr: *mut ()) -> Box<Self> () {
        Box::from_raw(ptr as *mut Self)
    }

    unsafe fn as_ref<'a>(ptr: *const ()) -> &'a Self {
        &*(ptr as *const Self)
    }

    unsafe fn as_mut<'a>(ptr: *mut ()) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}

macro_rules! impl_node_conv {
    ($ty:ty, $type_code:expr) => {
        impl RootNode for $ty {
            fn type_code() -> usize {
                $type_code
            }
        }
    }
}


impl_node_conv!(Leaf1, 1);
impl_node_conv!(Leaf2, 2);
impl_node_conv!(VecLeaf, 3);
impl_node_conv!(JPM, 4);

impl_root_ptr!(1 => Leaf1,
               2 => Leaf2,
               3 => VecLeaf,
               4 => JPM);
