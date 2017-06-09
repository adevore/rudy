use super::innerptr::{InnerPtr, IntoPtr};
use ::Key;
use ::rudymap::results::InsertResult;

pub trait JpmNode<K: Key, V> {
    type OverflowNode: IntoPtr<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V>;
    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V>;
    fn insert(&mut self, key: &[u8], value: V)
              -> InsertResult<V>;
    fn expand(self, key: &[u8], value: V) -> Box<Self::OverflowNode>;
}
