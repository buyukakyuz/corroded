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
pub mod pin;
pub mod race;
pub mod sync;
pub mod transmute;
pub mod uninit;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub mod backdoor;

pub mod prelude {
    pub use crate::aliasing::*;
    pub use crate::buffer::*;
    pub use crate::global::*;
    pub use crate::lifetime::*;
    pub use crate::memory::*;
    pub use crate::null::*;
    pub use crate::pin::*;
    pub use crate::race::*;
    pub use crate::sync::*;
    pub use crate::transmute::*;
    pub use crate::uninit::*;
    pub use crate::sync::*;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub use crate::backdoor::*;
}
