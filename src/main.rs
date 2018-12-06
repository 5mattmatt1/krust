#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(uniform_paths)]
#![no_std]
#![no_main]

mod vga_buffer;
mod serial;
mod pci;
mod rtl8139;
mod asm;

use core::panic::PanicInfo;
// use pci::scan_bus;

/*
 * Notes:
 * Check out the following files in the linux kernel info.
 * /net/ipv4/ *
 * /net/socket.c
 */

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
    use krust::pci::pci_slconf1_read;
    use krust::pci::pci_info_dump;
    use krust::pci::pci_parsedriver;
    use krust::rtl8139::RTL8139Driver;
    use krust::{serial_println};
    krust::gdt::init();
    krust::interrupts::init_idt();
    let mut rx_buffer: [u8; 1024] = [0; 1024];
    unsafe { PICS.lock().initialize() }; // new
    // let baseio_address: u32 = unsafe { pci_slconf1_read(0, 3, 0, 0x10) };
    // sprintln!("0x{:X}", baseio_address);
    // unsafe { pci_info_dump(0, 3) };
    
    // Dead RTL8139 code:
    // let success: bool = unsafe {krust::rtl8139::setup_rtl8139(baseio_address, &rx_buffer)};
    // println!("Sucessful driver bootup: {}", success);
    // let driver_info = pci_parsedriver()
    /*
    let class_str: &'static str = "";
    let subclass_str: &'static str = "";
    let prog_if_str: &'static str = "";
    pci_parsedriver(0x02, 0x00, 0x00, class_str, subclass_str, prog_if_str);
    println!("Class name: {}", class_str);
    */
    x86_64::instructions::interrupts::enable();
    let nic = RTL8139Driver::new(0, 3);
    unsafe {nic.hw_start(); }
    krust::hlt_loop();
}