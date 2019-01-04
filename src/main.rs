#![no_std]
#![feature(core_intrinsics, lang_items, asm)]
// #![no_main]

pub mod postman;
pub mod gpio;
pub mod uart;
pub mod vol;

const RPI_VECTOR_START: u32 = 0x0;

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
    // Older raspberry pi 1
    // use crate::postman::FrameBufferInfo;
    // let mut fb_info = FrameBufferInfo::new(640, 480, 24);
    // fb_info.render();
    // Newer raspberry pi 2
    use crate::postman::fb_init;
    fb_init();
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

/*
 * Look into what was going on in pigfx to make it work
 */
#[lang = "eh_personality"] extern fn eh_personality() {}
// #[lang = "panic_fmt"] extern fn panic_fmt(fmt: Arguments, file: &str, line: u32) {}

#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr1() {}


#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}

#[no_mangle]
pub extern fn prologue(table_start: isize, table_end: isize) {
    let vector: *mut u32 = RPI_VECTOR_START as *mut u32;

    let mut table = table_start;
    while table < table_end {
        let there = unsafe { vector.offset((table - table_start) / 4) };
        let here = table as *mut u32;

        unsafe { *there = *here; }

        table += 4;
    }
}

#[no_mangle]
pub unsafe extern fn _Unwind_Resume() {}

#[no_mangle]
// pub extern "C" fn _start() {
pub extern "C" fn main() {
    // test_uart();
    // test_led();
    use crate::gpio::enable_led;
    use crate::gpio::turn_on_led;
    use crate::uart::{uart_setup};
    enable_led();
    turn_on_led();
    unsafe {
        uart_setup();
        test_gpu();
        loop
        {
            asm!("nop");
        }
    }
}