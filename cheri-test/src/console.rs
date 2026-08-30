//! Console output over the QEMU virt machine PL011 UART

use critical_section::RawRestoreState;
use pl011_uart::Pl011Uart;

/// PL011 base address on QEMU `virt` machine
pub const UART0_BASE: usize = 0x0900_0000;

/// Configure the global writer used by `print!` and `println!`.
pub fn init() {
    // SAFETY: UART0_BASE is the PL011 MMIO region on the QEMU virt machine.
    if let Some(uart) = unsafe { Pl011Uart::from_address(UART0_BASE) } {
        pl011_uart::set_writer(uart);
    }
}

/// `critical_section::Impl` for a single core that never enables interrupts
struct SingleCore;
critical_section::set_impl!(SingleCore);

// SAFETY: The application runs on one core with interrupts disabled, so
// there is no other context that could enter a critical section concurrently.
unsafe impl critical_section::Impl for SingleCore {
    unsafe fn acquire() -> RawRestoreState {}
    unsafe fn release(_restore_state: RawRestoreState) {}
}
