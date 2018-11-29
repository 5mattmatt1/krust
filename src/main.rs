#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga_buffer;
mod serial;
mod pci;
mod rtl8139;

use core::panic::PanicInfo;
// use pci::scan_bus;

/* cargo rustc -- -Z pre-link-arg=-nostartfiles */

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    krust::hlt_loop();
}

// static HELLO: &[u8] = b"Hello World!";

/*

*/
/*
pub fn enumerate_buses()
{
    use krust::pci::check_vendor;
    use krust::pci::get_device;
    use krust::pci::get_bar0;
    use krust::pci::get_bar1;
    use krust::pci::get_bar2;
    use krust::pci::get_bar3;
    let mut vendor: u32;
    let mut good_vendors: u16 = 0;
    for bus in 0..255
    {
        for slot in 0..255
        {
            vendor = check_vendor(bus, slot);
            if vendor != 0xFFFFFFFF && vendor != 0x12378086
            {
                good_vendors += 1;
                serial_println!("Bus: {}", bus);
                serial_println!("Slot: {}", slot);
                serial_println!("Vendor: 0x{:X}", vendor);
                serial_println!("Device: 0x{:X}", get_device(bus, slot));
                serial_println!("BAR0: 0x{:X}", get_bar0(bus, slot));
                serial_println!("BAR1: 0x{:X}", get_bar1(bus, slot));
                serial_println!("BAR2: 0x{:X}", get_bar2(bus, slot));
                serial_println!("BAR3: 0x{:X}", get_bar3(bus, slot));
            }
        }
    }
    serial_println!("Number of good vendors: {}", good_vendors);
}
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use krust::interrupts::PICS;
    // use krust::pci::pci_conf1_read;
    use krust::pci::pci_slconf1_read;
    // use krust::pci::pci_config_read_word;
    // use krust::pci::check_vendor;
    /*
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */
    // vga_buffer::print_something();
    krust::gdt::init();
    krust::interrupts::init_idt();
    // println!("Hello World{}", "!");
    // serial_println!("Hello Host{}", "!");
    // panic!("Some panic message");
    // unsafe { exit_qemu(); }
    // Invokes breakpoint handler.
    // x86_64::instructions::int3();
    let mut rx_buffer: [u8; 1024] = [0; 1024];
    unsafe { PICS.lock().initialize() }; // new
    let baseio_address: u32 = unsafe { pci_slconf1_read(0, 3, 0, 0x10) };
    println!("{}", rx_buffer[0]);
    let success: bool = unsafe {krust::rtl8139::setup_rtl8139(baseio_address, &rx_buffer)};
    println!("Sucessful driver bootup: {}", success);
    if success
    {
        for i in rx_buffer.iter() 
        {
            if rx_buffer[*i as usize] != 0
            {
                println!("i: {}, rx_buffer[i]: {}", i, rx_buffer[0]);
            }
        }
    }
    x86_64::instructions::interrupts::enable();
    krust::hlt_loop();
}