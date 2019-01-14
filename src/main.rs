#![no_std]
#![feature(core_intrinsics, lang_items, asm)]
// #![no_main]

pub mod postman;
pub mod gpio;
pub mod uart;
pub mod vol;
pub mod memory;
pub mod font;
pub mod font0;

const RPI_VECTOR_START: u32 = 0x0;

fn test_gpu()
{
    // Older raspberry pi 1
    // use crate::postman::FrameBufferInfo;
    // let mut fb_info = FrameBufferInfo::new(640, 480, 24);
    // fb_info.render();
    // Newer raspberry pi 2
    use crate::postman::fb_initb;
    fb_initb();
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
    use crate::gpio::{enable_led, turn_on_led, output_gpio, setup_gpio};
    use crate::uart::{uart_setup};
    enable_led();
    turn_on_led();
    // Test gpio later...
    // setup_gpio(18, crate::gpio::OUTPUT, crate::gpio::PUD_OFF);
    // output_gpio(18, true); // Testing red LED
    /*
     * Add this to the list of research links on GitHub:
     * https://github.com/bztsrc/raspi3-tutorial.
     */
    unsafe {
        uart_setup();
        test_gpu();
        loop
        {
            asm!("nop");
        }
    }
}