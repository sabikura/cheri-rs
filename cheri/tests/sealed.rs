#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cheri_test::runner)]
#![reexport_test_harness_main = "test_main"]

use aarch64_purecap_rt::{entry, exception};
use cheri::prelude::*;
use cheri::ptr::Seal;

#[test_case]
fn seal_roundtrip() {
    let mut x = 5u32;
    let seal = Seal::new(cheri::ptr::default_data_mut(), 42..43);
    let ptr = (&mut x as *mut u32).seal(seal);
    assert!(ptr.tag());
    assert!(ptr.is_sealed());
    assert_eq!(ptr.otype(), 42);
    let unsealed = ptr.unseal(seal);
    assert!(unsealed.tag());
    assert!(!unsealed.is_sealed());
    unsafe {
        assert_eq!(*unsealed, 5);
    }
}

#[test_case]
fn seal_twice_clears_tag() {
    let x = 5u32;
    let seal = Seal::new(cheri::ptr::default_data_mut(), 42..44);
    let ptr = (&x as *const u32).seal(seal);
    let twice = ptr.seal(seal.with_otype(43));
    assert!(!twice.tag());
}

#[test_case]
fn seal_outside_pool_clears_tag() {
    let x = 5u32;
    let seal = Seal::new(cheri::ptr::default_data_mut(), 42..43);
    let ptr = (&x as *const u32).seal(seal.with_otype(50));
    assert!(!ptr.tag());
}

#[test_case]
fn sealed() {
    #[derive(cheri::Sealed)]
    struct SealedStruct {}

    #[cheri::sealed]
    impl SealedStruct {}
}

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
