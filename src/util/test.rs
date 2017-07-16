use std::sync::atomic::{AtomicUsize,Ordering};

#[derive(Debug)]
pub struct Droppable<'a>(pub &'a AtomicUsize);
impl<'a> Drop for Droppable<'a> {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}
