#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![no_std] // don't link the Rust standard library

// NEW: We need to add `pub` here to make them accessible from the outside
pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod pci;
pub mod rtl8139;
pub mod asm;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}