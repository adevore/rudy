#[must_use]
pub enum InsertResult<V> {
    Success,
    Resize(V)
}

impl<V> InsertResult<V> {
    /// Attach a success invariant
    pub fn success(self) {
        match self {
            InsertResult::Success => panic!("Unexpected insertion overflow"),
            InsertResult::Resize(_) => ()
        }
    }
}

