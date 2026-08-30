#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cheri_test::runner)]
#![reexport_test_harness_main = "test_main"]

use aarch64_purecap_rt::{entry, exception};
use cheri::prelude::*;
use cheri::ptr::Perms;

#[entry]
fn main() -> ! {
    test_main();
    cheri_test::qemu_exit(0)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cheri_test::panic(info)
}

#[exception(Sync, EL1)]
fn sync_exception() {
    cheri_test::sync_exception()
}

#[test_case]
fn ddc_has_load_and_store() {
    let perms = cheri::ptr::default_data::<u8>().perms();
    assert!(perms.contains(Perms::LOAD | Perms::STORE));
}

#[test_case]
fn perms_contains_empty() {
    assert!(Perms::empty().contains(Perms::empty()));
    assert!(Perms::all().contains(Perms::LOAD | Perms::STORE | Perms::EXECUTE));
    assert!(!Perms::empty().contains(Perms::LOAD));
}

#[test_case]
fn clear_except_is_monotonic() {
    let ptr = cheri::ptr::default_data_mut::<u8>();
    let reduced = ptr.with_perms_clear_except(Perms::LOAD);
    assert!(reduced.perms().contains(Perms::LOAD));
    assert!(!reduced.perms().contains(Perms::STORE));
    let restored = reduced.with_perms_clear_except(Perms::all());
    assert!(!restored.perms().contains(Perms::STORE));
}

#[test_case]
fn clear_except_empty_keeps_nothing() {
    let ptr = cheri::ptr::default_data::<u8>().with_perms_clear_except(Perms::empty());
    assert!(!ptr.perms().contains(Perms::LOAD));
    assert!(!ptr.perms().contains(Perms::STORE));
}

#[test_case]
fn bounds_keep_value_accessible() {
    let mut buf = [1u8, 2, 3, 4];
    let ptr = buf.as_mut_ptr().with_bounds(buf.len());
    unsafe {
        assert_eq!(*ptr, 1);
        assert_eq!(*ptr.add(3), 4);
        *ptr.add(1) = 9;
    }
    assert_eq!(buf[1], 9);
}

#[test_case]
fn exact_bounds_keep_value_accessible() {
    let buf = [7u32; 8];
    let ptr = buf.as_ptr().with_bounds_exact(core::mem::size_of_val(&buf));
    unsafe {
        assert_eq!(*ptr, 7);
        assert_eq!(*ptr.add(7), 7);
    }
}
