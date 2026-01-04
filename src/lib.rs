#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![no_std]

extern crate alloc;

pub mod aliasing;
pub mod buffer;
pub mod global;
pub mod lifetime;
pub mod memory;
pub mod null;
pub mod race;
pub mod transmute;
pub mod uninit;
pub mod sync;

pub mod prelude {
    pub use crate::aliasing::*;
    pub use crate::buffer::*;
    pub use crate::global::*;
    pub use crate::lifetime::*;
    pub use crate::memory::*;
    pub use crate::null::*;
    pub use crate::race::*;
    pub use crate::transmute::*;
    pub use crate::uninit::*;
    pub use crate::sync::*;
}
