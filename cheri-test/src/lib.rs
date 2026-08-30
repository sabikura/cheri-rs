#![no_std]

mod console;
mod fault;

pub use fault::sync_exception;

use core::panic::PanicInfo;
use owo_colors::OwoColorize;
use pl011_uart::{print, println};

/// Something the test runner can execute (implemented only on [`Fn`] types)
pub trait Testable {
    fn run(&self);
}

impl<F: Fn()> Testable for F {
    /// Provide a nice message with the test name and the status
    fn run(&self) {
        print!("{}... ", core::any::type_name::<F>());
        self();
        println!("{}", "ok".green());
    }
}

/// Run all `#[test_case]` items of the crate.
pub fn runner(tests: &[&dyn Testable]) {
    console::init();
    fault::init();
    println!();
    println!("running {} test(s)", tests.len());
    println!();
    for test in tests {
        test.run();
    }
    println!();
    println!("{}", "all tests passed".green());
}

/// Panic handler for test crates that reports the panic info and
/// quits QEMU with exit code 1.
pub fn panic(info: &PanicInfo) -> ! {
    console::init();
    println!("test failed with: \n{}", info.red());
    qemu_exit(1)
}

/// Terminate QEMU with the given exit code using semihosting
pub fn qemu_exit(code: u64) -> ! {
    const SYS_EXIT: u64 = 0x18;
    const ADP_STOPPED_APPLICATION_EXIT: u64 = 0x20026;

    let block = [ADP_STOPPED_APPLICATION_EXIT, code];
    loop {
        // SAFETY: Semihosting call
        unsafe {
            core::arch::asm!(
                "hlt #0xf000",
                in("x0") SYS_EXIT,
                in("x1") block.as_ptr() as usize,
            );
        }
    }
}
