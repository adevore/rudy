use super::rootptr::RootPtr;
use ::Key;

#[must_use]
pub enum InsertResult<V> {
    Success(Option<V>),
    Resize(V)
}

impl<V> InsertResult<V> {
    /// Attach a success invariant
    pub fn success(self) -> Option<V> {
        match self {
            InsertResult::Success(evicted) => evicted,
            InsertResult::Resize(_) => panic!("Unexpected insertion overflow")
        }
    }

    pub fn replace(place: &mut V, value: V) -> InsertResult<V> {
        let old_value = ::std::mem::replace(place, value);
        InsertResult::Success(Some(old_value))
    }
}

#[must_use]
pub enum RemoveResult<V> {
    /// The entry was removed without needing a downsize
    Success(Option<V>),
    /// Eviction requires a node downsize
    Downsize
}

impl<V> RemoveResult<V> {
    /// Attach a success invariant
    pub fn success(self) -> Option<V> {
        match self {
            RemoveResult::Success(evicted) => evicted,
            RemoveResult::Downsize => panic!("Unexpected remove underflow")
        }
    }
}
