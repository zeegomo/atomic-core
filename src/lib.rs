#![no_std]
pub use core::*;

#[cfg(not(target_has_atomic))]
pub mod sync {
    pub mod atomic {
        pub use portable_atomic::*;
    }
}
