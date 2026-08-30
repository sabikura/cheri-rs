//! CHERI capabilities as raw pointers.
//!
//! This module extends the raw pointers type on pure capability targets by implementing the
//! [`CheriPtr`] trait. Capability-aware pointers carry bounds, permissions, and an out-of-band
//! validity tag alongside the address; a dereference traps unless the capability is tagged,
//! unsealed, in bounds, and holds the permission the access requires.
//!
//! This module provides access to the Default Data Capability with the [`default_data`] and [`default_data_mut`]
//! functions.

/// Returns the Default Data Capability as a constant raw pointer.
///
/// The returned pointer carries the DDC's tag, bounds, and permissions. Its **address is the
/// DDC's address, not the start of its bounds**, and on most platforms that address is zero.
/// The pointer is therefore not dereferenceable as-is: set an address within the bounds
/// before use.
pub fn default_data<T: Sized>() -> *const T {
    unsafe { crate::intrinsics::__cheri_ddc_get() as *mut T as *const T }
}

/// Returns the Default Data Capability (DDC) as a mutable raw pointer.
///
/// Identical to [`crate::ptr::default_data`] in every aspect except the returned pointer type.
pub fn default_data_mut<T: Sized>() -> *mut T {
    unsafe { crate::intrinsics::__cheri_ddc_get() as *mut T }
}

/// A set of capability permissions.
///
/// Permissions are only ever meaningful alongside a valid tag: an untagged capability may still
/// report permission bits, but they authorise nothing. All operations that change the
/// permissions of a live capability are monotonic.
///
/// # Example
///
/// ```
/// use cheri::prelude::*;
///
/// let perms = Perms::UNSEAL | Perms::STORE_LOCAL;
/// let ptr = cheri::ptr::default_data_mut().with_perms(perms);
/// ```
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perms(u64);

impl Perms {
    /// Unseal a sealed capability
    pub const UNSEAL: Self = Perms(1 << 10);
    /// Seal an unsealed capability
    pub const SEAL: Self = Perms(1 << 11);
    /// Store a Local capability to memory
    pub const STORE_LOCAL: Self = Perms(1 << 12);
    /// Store a valid capability from a capability register
    pub const STORE_CAPABILITY: Self = Perms(1 << 13);
    /// Load a valid capability to a capability register
    pub const LOAD_CAPABILITY: Self = Perms(1 << 14);
    /// Execute instructions
    pub const EXECUTE: Self = Perms(1 << 15);
    /// Store to memory
    pub const STORE: Self = Perms(1 << 16);
    /// Load from memory
    pub const LOAD: Self = Perms(1 << 17);

    /// Check whether this set contains a permission, or every permission in a set.
    ///
    /// Returns `true` if all bits of `other` are present. Note that this means
    /// `perms.contains(Perms::empty())` is always `true`, and that for a multi-bit `other` a
    /// `false` result does not say which bit was missing.
    ///
    /// # Example
    ///
    /// ```
    /// use cheri::prelude::*;
    ///
    /// let perms = cheri::ptr::default_data().perms();
    /// assert!(perms.contains(Perms::LOAD | Perms::STORE));
    /// ```
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Returns a set containing every permission defined by this type.
    pub fn all() -> Self {
        Self(u64::MAX)
    }

    /// Returns an empty set of permissions.
    pub fn empty() -> Self {
        Self(0)
    }
}

impl core::ops::BitOr for Perms {
    type Output = Perms;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}

pub trait CheriPtr {
    /// Set the bounds of the capability.
    ///
    /// This will change the bounds of capability with address `addr` into [`addr`, `addr` +
    /// `len`). If the capability bounds cannot be represented into the implementation-specific
    /// encoding, they are rounded.
    ///
    /// Alignment requirements are encoding-dependent and might cause the tag to clear depending on
    /// the architecture.
    fn with_bounds(self, len: usize) -> Self;
    /// Set the exact bounds of the capability.
    ///
    /// This will change the bounds of capability with address `addr` into [`addr`, `addr` +
    /// `len`). If the capability bounds cannot be represented into the implementation-specific
    /// encoding, this will clear the capability tag.
    ///
    /// Alignment requirements are encoding-dependent and might cause the tag to clear depending on
    /// the architecture.
    fn with_bounds_exact(self, len: usize) -> Self;
    /// Clear every permission not present in `perms`, i.e. intersect the capability's current
    /// permissions with `perms`.
    ///
    /// This operation is monotonic. Bits set in `perms` that the capability does not already
    /// hold are ignored, so the result is always a subset of the permissions held on entry.
    /// Passing [`Perms::all()`] is a no-op, and passing [`Perms::empty()`] leaves a capability
    /// that still carries a valid tag and bounds but authorises nothing.
    ///
    /// The address and bounds are unchanged. The tag is cleared if the capability is sealed,
    /// since permissions cannot be altered through a seal.
    fn with_perms_clear_except(self, perms: Perms) -> Self;
    /// Return the permissions currently held by this capability.
    fn perms(&self) -> Perms;
}

/// Implement [`CheriPtr`] trait for a pointer type. Both *const and *mut have the same implementation.
///
/// Implementations of the trait are just wrappers over the LLVM intrinsics defined in
/// [`crate::intrinsics`].
macro_rules! impl_cheri_ptr {
    ($ptr:ty) => {
        impl<T: Sized> CheriPtr for $ptr {
            fn with_bounds(self, len: usize) -> Self {
                unsafe {
                    crate::intrinsics::__cheri_cap_bounds_set(self as *mut u8, len as u64) as $ptr
                }
            }

            fn with_bounds_exact(self, len: usize) -> Self {
                unsafe {
                    crate::intrinsics::__cheri_cap_bounds_set_exact(self as *mut u8, len as u64)
                        as $ptr
                }
            }

            fn with_perms_clear_except(self, perms: Perms) -> Self {
                unsafe {
                    crate::intrinsics::__cheri_cap_perms_and(self as *mut u8, perms.0) as $ptr
                }
            }

            fn perms(&self) -> Perms {
                Perms(unsafe { crate::intrinsics::__cheri_cap_perms_get(*self as *mut u8) })
            }
        }
    };
}

impl_cheri_ptr!(*mut T);
impl_cheri_ptr!(*const T);
