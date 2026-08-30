use crate::console;
use core::arch::asm;
use owo_colors::OwoColorize;
use pl011_uart::println;

pub(crate) fn init() {
    unsafe {
        asm!(
            "mrs {tmp}, cctlr_el1",
            "orr {tmp}, {tmp}, #(1 << 5)",
            "msr cctlr_el1, {tmp}",
            "isb",
            tmp = out(reg) _,
            options(nomem, nostack),
        );
    }
}

pub fn sync_exception() -> ! {
    console::init();
    let esr: u64;
    let elr: u64;
    let far: u64;
    unsafe {
        asm!(
            "mrs {esr}, esr_el1",
            "mrs {elr}, elr_el1",
            "mrs {far}, far_el1",
            esr = out(reg) esr,
            elr = out(reg) elr,
            far = out(reg) far,
            options(nomem, nostack),
        );
    }
    let class = esr >> 26;
    let iss = esr & 0x1ff_ffff;
    println!("{}", "test faulted".red());
    println!("  {}", describe(class, iss).red());
    println!("  esr: {esr:#018x}");
    println!("  elr: {elr:#018x}");
    println!("  far: {far:#018x}");
    crate::qemu_exit(1)
}

fn describe(class: u64, iss: u64) -> &'static str {
    match class {
        0x00 => "unknown reason",
        0x0e => "illegal execution state",
        0x15 => "svc",
        0x18 => "trapped system register access",
        0x20 | 0x21 => "instruction abort",
        0x22 => "pc alignment fault",
        0x24 | 0x25 => data_abort(iss & 0x3f),
        0x26 => "sp alignment fault",
        0x2c => "floating-point exception",
        0x2f => "serror",
        0x30 | 0x31 => "breakpoint",
        0x32 | 0x33 => "software step",
        0x34 | 0x35 => "watchpoint",
        0x3c => "brk instruction",
        _ => "unrecognised exception class",
    }
}

fn data_abort(fsc: u64) -> &'static str {
    match fsc {
        0x00..=0x03 => "data abort: address size fault",
        0x04..=0x07 => "data abort: translation fault",
        0x09..=0x0b => "data abort: access flag fault",
        0x0d..=0x0f => "data abort: permission fault",
        0x10 => "data abort: synchronous external abort",
        0x21 => "data abort: alignment fault",
        0x28 => "data abort: capability tag fault",
        0x29 => "data abort: capability sealed fault",
        0x2a => "data abort: capability bounds fault",
        0x2b => "data abort: capability permission fault",
        _ => "data abort",
    }
}
