//! Raw bindings to the CHERI-LLVM intrinsics declared in `llvm/include/llvm/IR/IntrinsicsCHERICap.td`

extern "C" {
    #[link_name = "llvm.cheri.cap.length.get.i64"]
    pub(crate) fn __cheri_cap_length_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.base.get.i64"]
    pub(crate) fn __cheri_cap_base_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.perms.get.i64"]
    pub(crate) fn __cheri_cap_perms_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.flags.get.i64"]
    pub(crate) fn __cheri_cap_flags_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.type.get.i64"]
    pub(crate) fn __cheri_cap_type_get(cap: *mut u8) -> i64;

    #[link_name = "llvm.cheri.cap.offset.get.i64"]
    pub(crate) fn __cheri_cap_offset_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.address.get.i64"]
    pub(crate) fn __cheri_cap_address_get(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.copy.from.high.i64"]
    pub(crate) fn __cheri_cap_copy_from_high(cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.diff.i64"]
    pub(crate) fn __cheri_cap_diff(a: *mut u8, b: *mut u8) -> i64;

    #[link_name = "llvm.cheri.cap.bounds.set.i64"]
    pub(crate) fn __cheri_cap_bounds_set(cap: *mut u8, len: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.bounds.set.exact.i64"]
    pub(crate) fn __cheri_cap_bounds_set_exact(cap: *mut u8, len: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.perms.and.i64"]
    pub(crate) fn __cheri_cap_perms_and(cap: *mut u8, mask: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.flags.set.i64"]
    pub(crate) fn __cheri_cap_flags_set(cap: *mut u8, flags: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.offset.set.i64"]
    pub(crate) fn __cheri_cap_offset_set(cap: *mut u8, offset: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.address.set.i64"]
    pub(crate) fn __cheri_cap_address_set(cap: *mut u8, address: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.copy.to.high.i64"]
    pub(crate) fn __cheri_cap_copy_to_high(cap: *mut u8, high: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.tag.get"]
    pub(crate) fn __cheri_cap_tag_get(cap: *mut u8) -> bool;

    #[link_name = "llvm.cheri.cap.sealed.get"]
    pub(crate) fn __cheri_cap_sealed_get(cap: *mut u8) -> bool;

    #[link_name = "llvm.cheri.cap.subset.test"]
    pub(crate) fn __cheri_cap_subset_test(parent: *mut u8, child: *mut u8) -> bool;

    #[link_name = "llvm.cheri.cap.equal.exact"]
    pub(crate) fn __cheri_cap_equal_exact(a: *mut u8, b: *mut u8) -> bool;

    #[link_name = "llvm.cheri.cap.tag.clear"]
    pub(crate) fn __cheri_cap_tag_clear(cap: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.seal"]
    pub(crate) fn __cheri_cap_seal(cap: *mut u8, sealer: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.conditional.seal"]
    pub(crate) fn __cheri_cap_conditional_seal(cap: *mut u8, sealer: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.unseal"]
    pub(crate) fn __cheri_cap_unseal(cap: *mut u8, sealer: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.seal.entry"]
    pub(crate) fn __cheri_cap_seal_entry(cap: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.build"]
    pub(crate) fn __cheri_cap_build(auth: *mut u8, bits: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.type.copy"]
    pub(crate) fn __cheri_cap_type_copy(dst: *mut u8, src: *mut u8) -> *mut u8;

    #[link_name = "llvm.cheri.cap.perms.check.i64"]
    pub(crate) fn __cheri_cap_perms_check(cap: *mut u8, perms: u64);

    #[link_name = "llvm.cheri.cap.type.check"]
    pub(crate) fn __cheri_cap_type_check(cap: *mut u8, sealer: *mut u8);

    #[link_name = "llvm.cheri.stack.cap.get"]
    pub(crate) fn __cheri_stack_cap_get() -> *mut u8;

    #[link_name = "llvm.cheri.ddc.get"]
    pub(crate) fn __cheri_ddc_get() -> *mut u8;

    #[link_name = "llvm.cheri.pcc.get"]
    pub(crate) fn __cheri_pcc_get() -> *mut u8;

    #[link_name = "llvm.cheri.cap.to.pointer.i64"]
    pub(crate) fn __cheri_cap_to_pointer(root: *mut u8, cap: *mut u8) -> u64;

    #[link_name = "llvm.cheri.cap.from.pointer.i64"]
    pub(crate) fn __cheri_cap_from_pointer(root: *mut u8, ptr: u64) -> *mut u8;

    #[link_name = "llvm.cheri.cap.from.pointer.nonnull.zero.i64"]
    pub(crate) fn __cheri_cap_from_pointer_nonnull_zero(root: *mut u8, ptr: u64) -> *mut u8;

    #[link_name = "llvm.cheri.round.representable.length.i64"]
    pub(crate) fn __cheri_round_representable_length(len: u64) -> u64;

    #[link_name = "llvm.cheri.representable.alignment.mask.i64"]
    pub(crate) fn __cheri_representable_alignment_mask(len: u64) -> u64;
}
