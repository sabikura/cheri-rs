#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cheri_test::runner)]
#![reexport_test_harness_main = "test_main"]

use aarch64_purecap_rt::{entry, exception};

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

// Test case that should also fail, used to test that `sync_exception` works :)
// #[test_case]
// fn deref_forged_pointer() {
//     let value = 42u32;
//     let addr = &value as *const u32 as usize;
//     let forged = addr as *const u32;
//     let read = unsafe { core::ptr::read_volatile(forged) };
//     assert_eq!(read, 42);
// }
