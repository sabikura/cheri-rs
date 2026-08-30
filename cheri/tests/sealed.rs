#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cheri_test::runner)]
#![reexport_test_harness_main = "test_main"]

use aarch64_purecap_rt::{entry, exception};

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
