use std::ptr;
use std::marker::PhantomType;
use ::rudymap::results::InsertResult;

const DECODE_POP_WIDTH: usize = usize::BYTES - 1;

enum InnerPtrType {
    Empty = 1
}

#[derive(Clone, Copy)]
pub struct InnerPtr<T> {
    ptr: *mut T,
    meta: usize,
    marker: PhantomType<T>
}

impl InnerPtr<T> {
    pub const fn empty(&self) -> InnerPtr<T> {
        InnerPtr {
            ptr: ptr::null_mut,
            meta: 0,
            marker: PhantomType
        }
    }

    pub fn get_decode_pop(&self, depth: usize) -> (&[u8], usize) {
        let (decode, pop) = self.decode_pop.split_at(depth);
    }

    pub fn get_population(&self, depth: usize) -> usize {
        self.get_decode_pop(depth).1
    }

    pub fn set_population(&self, depth: usize, value: usize) {
        let population_field = &mut self.decode_pop[depth..];
        util::partial_write(population_field, value).unwrap();
    }

    pub fn decoded(&self, depth: usize) -> usize {
        self.get_decode_pop(depth).0
    }

    fn as_ref(&self) -> InnerPtrRef<'a, T> {
        let ptr = self.ptr & !0x7;
        let type_id = self.ptr & 0x7; match type_id {
            1 => Empty,
            2 => {
                let ptr = ptr as *const BranchLinear<T>;
                InnerPtrRef::BranchLinear(unsafe { &*ptr })
            },
            3 => {
                let ptr = ptr as *const BranchBitmap<T>;
                InnerPtrRef::BranchBitmap(unsafe { &*ptr })
            },
            4 => {
                let ptr = ptr as *const BranchUncompressed<T>;
                InnerPtrRef::BranchUncompressed(unsafe{ &*ptr })
            },
            5 => {
                let ptr = ptr as *const LeafLinear<T>;
                InnerPtrRef::LeafLinear(unsafe{ &*ptr })
            },
            6 => {
                let ptr = ptr as *const LeafBitmap<T>;
                InnerPtrRef::LeafBitmap(unsafe{ &*ptr })
            },
            invalid => {
                panic!("Unknown type ID {}", invalid);
            }
        }
    }

    fn as_mut_ref(&mut self) -> InnerPtrMutRef<'a, T> {
        
    }
    
}

enum InnerPtrRef<'a, T> {
    Empty,
    BranchLinear(&'a BranchLinear<T>),
    BranchBitmap(&'a BranchBitmap<T>),
    BranchUncompressed(&'a BranchUncompressed<T>),
    LeafLinear(&'a LeafLinear<T>),
    LeafBitmap(&'a LeafBitmap<T>),
}

