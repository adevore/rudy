use ::util::locksteparray::LockstepArray;
use super::innerptr::InnerPtr;
use ::Key;
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use super::branch_bitmap::BranchBitmap;

pub struct BranchLinear<K: Key, V> {
    array: LockstepArray<[u8; 7], [InnerPtr<K, V>; 7]>
}

impl<K: Key, V> BranchLinear<K, V> {
    pub fn new() -> BranchLinear<K, V> {
        BranchLinear {
            array: Default::default()
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for BranchLinear<K, V> {
    type OverflowNode = BranchBitmap<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!();
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        unimplemented!();
    }

    fn expand(self, key: &[u8], value: V) -> Box<BranchBitmap<K, V>> {
        unimplemented!();
    }
}
