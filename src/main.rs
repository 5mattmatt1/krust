#![no_std]
#![feature(core_intrinsics, lang_items, asm)]
#![no_main]

pub mod postman;
pub mod gpio;

/*
use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
*/

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
/*
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}
*/

/*
fn uart_init()
{
    // Disablo UART0
    mmio_write(UART_CR, 0x00000000);

    // Setup the GPIO pin 14 && 15

    // Disable pull up/down for all GPIO pins & delay for 150 cycles
    mmio_write(GPPUD, 0x00000000);
}

fn test_uart()
{
    write("Hello Rust Kernel world!");
    loop {
        writec(getc())
    }
}
*/

fn test_gpu()
{
    use crate::postman::FrameBufferInfo;
    let mut fb_info = FrameBufferInfo::new(640, 480, 24);
    fb_info.render();
}

/*
fn test_invalid_frame_buffer()
{
    use crate::postman::FrameBufferInfo;
    let mut fbInfo = FrameBufferInfo::new();
}
*/

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop
    {
        unsafe { asm!("nop") } ;
    }
}

#[no_mangle]
pub extern "C" fn _start() {
    // test_uart();
    // test_led();
    use crate::gpio::enable_led;
    use crate::gpio::turn_on_led;
    use crate::gpio::sleep;
    enable_led();
    turn_on_led();
    sleep(500000);
    test_gpu();
}