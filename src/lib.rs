#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(conservative_impl_trait)]
#![feature(associated_consts)]
#![feature(slice_patterns)]

extern crate nodrop;
extern crate num_traits;

mod util;
pub mod rudymap;
mod key;

pub use key::Key;
//mod rudyset;

//pub use rudyset::RudySet;
//pub use rudymap::RudyMap;

