//! Capability-aware pointer operations for CHERI targets.
//!
//! CHERI replaces integer pointers with *capabilities*: an address carried alongside bounds,
//! permissions, and an out-of-band validity tag that hardware maintains and software cannot
//! forge.

#![no_std]
#![feature(link_llvm_intrinsics)]

mod intrinsics;
pub mod ptr;

/// The CHERI prelude
///
/// # Contents:
/// - [`crate::ptr::CheriPtr`] trait that provides capability-aware methods for raw pointers
pub mod prelude {
    pub use super::ptr::CheriPtr;
}
