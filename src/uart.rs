/*
 * MIT License
 *
 * Copyright (c) 2018-2019 Matthew Henderson <mattw2018@hotmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// TODO:
// Make a simple UART logger to increase
// my ability to debug
use crate::vol::{write32, read32, wor32};

/* Should be part off gpio.rs */
const GPFSEL1: u32 =    0x3F200004;
// const GPSET0: u32 =     0x3F20001C;
// const GPCLR0: u32 =     0x3F200028;
const GPPUD: u32 =      0x3F200094;
const GPPUDCLK0: u32 =  0x3F200098;

// Actual uart
const AUX_ENABLES: u32 =    0x3F215004;
const AUX_MU_IO_REG: u32 =  0x3F215040;
const AUX_MU_IER_REG: u32 = 0x3F215044;
const AUX_MU_IIR_REG: u32 = 0x3F215048;
const AUX_MU_LCR_REG: u32 = 0x3F21504C;
const AUX_MU_MCR_REG: u32 = 0x3F215050;
const AUX_MU_LSR_REG: u32 = 0x3F215054;
// const AUX_MU_MSR_REG: u32 = 0x3F215058;
// const AUX_MU_SCRATCH: u32 = 0x3F21505C;
const AUX_MU_CNTL_REG: u32 = 0x3F215060;
// const AUX_MU_STAT_REG: u32 = 0x3F215064;
const AUX_MU_BAUD_REG: u32 = 0x3F215068;

// GPIO14 TXD0 and TXD1
// GPIO15 RXD0 and RXD1
// alt function 5 for uart1
// alt function 0 for uart0

// 7 << 12 // gpio14
// 2 << 12 // alt5

#[no_mangle]
pub unsafe fn uart_setup()
{
    wor32(AUX_ENABLES, 1);
    write32(AUX_MU_IER_REG, 0);
    write32(AUX_MU_CNTL_REG, 0);
    write32(AUX_MU_LCR_REG, 3);
    write32(AUX_MU_MCR_REG, 0);
    write32(AUX_MU_IER_REG, 0);
    write32(AUX_MU_IIR_REG, 0xC6);
    write32(AUX_MU_BAUD_REG, 270);

    let mut ra = read32(GPFSEL1);    
    
    ra &= !((7 << 12) | (7 << 15));
    ra |= (2 << 12) | (2 << 15); // alt5
    
    write32(GPFSEL1, ra);

    write32(GPPUD, 0);
    for _ in 0..150
    {
        asm!("nop");
    }
    
    /* Usefulness? */
    write32(GPPUDCLK0, (1 << 14) | (1 << 15));
    for _ in 0..150
    {
        asm!("nop");
    }
    write32(GPPUDCLK0, 0);

    write32(AUX_MU_CNTL_REG, 3);
}

#[no_mangle]
pub fn uart_putc(c: char)
{

    unsafe {
        while (read32(AUX_MU_LSR_REG) & 0x20) == 0
        {
            asm!("nop"); 
        }
        write32(AUX_MU_IO_REG, c as u32);
    }
}

pub fn uart_putc_ascii(mut c: char)
{
    if (c as u8) < 0x20 || (c as u8) > 0x7F
    {
        c = '.';
    }
    uart_putc(c);
}

#[no_mangle]
pub fn uart_getc() -> char
{
    unsafe {
        while (read32(AUX_MU_LSR_REG) & 0x01) == 0
        {
            asm!("nop");
        }

        return read32(AUX_MU_IO_REG) as u8 as char;
    }
}

// Fairly safe
#[no_mangle]
pub fn uart_puts(s: &str)
{
    unsafe 
    {
        // uart_putc((s.len() + 0x30) as u8 as char);
        for byte in s.as_bytes()
        {
            uart_putc(*byte as char);
            /*
            match *byte
            {
                    0x20...0x7e | b'\n' => uart_putc(byte as char),
                    _ => uart_putc(0x30 as char),
            }
            */
        }
    }
}

// Not a very fast modulo...
/*
pub fn modulo(den: usize, nom: usize) -> usize
{
    let r0 = den / nom;
    // let r1 = r0 * nom;
    // return r1 - den;
    return r0;
}
*/

#[no_mangle]
pub fn uart_writeaddr(mut addr: usize) -> i8
{
    uart_hex(addr);
    uart_putc('\n');
    return 0;
}

pub fn uart_hex8(mut byte: u8) -> i8
{
    let mut delta: u8;
    let mut drepr: u8;
    let mut modulo: u8;
    let pow_ref: [u8; 3] = [0x0, 0xF, 0xFF];

    let mut msg_arry: [u8; 2] = [0; 2];
    for i in 1..3
    {    
        delta = byte & pow_ref[i as usize];
        byte -= delta;
        drepr = delta >> ((i - 1) << 2);
        msg_arry[2-i] = drepr as u8;
    }

    for i in 0..2
    {
        drepr = msg_arry[i] as u8;
        if drepr >= 10 && drepr < 16
        {
            uart_putc((0x37 + drepr) as u8 as char);
        }
        else if drepr < 10
        {
            uart_putc((0x30 + drepr) as u8 as char);
        }
    }
    return 0;
}

pub fn uart_hex(mut addr: usize) -> i8
{
    // Sadly, this prints in reverse
    // Maximum u32 in hex: 0xFFFF_FFFF
    // addr = reverseBits(addr as u32) as usize;
    let mut delta: usize;
    let mut drepr: usize;
    let mut modulo: usize;
    let pow_ref: [usize; 9] = [0x0, 0xF, 0xFF, 0xFFF, 0xFFFF, 0xFFFFF, 0xFFFFFF, 0xFFFFFFF, 0xFFFFFFFF];

    let mut msg_arry: [u8; 8] = [0; 8];
    for i in 1..9
    {
        // a % b == a & (b - 1)
        // Where b % 2 == 0
        delta = addr & pow_ref[i as usize];
        // uart_putc('c');
        addr -= delta;
        // uart_putc('d');
        drepr = delta >> ((i-1) << 2);
        // a << 2 == a * 4, 
        // but bitshift method is faster
        msg_arry[8-i] = drepr as u8;
    }

    uart_putc('0');
    uart_putc('x');
    for i in 0..8
    {
        drepr = msg_arry[i] as usize;
        if drepr >= 10 && drepr < 16
        {
            // 0x37 + 0xA == 'A'
            // 0x37 + 0xB == 'B'
            // etc.
            uart_putc((0x37 + drepr) as u8 as char);
        }
        else if drepr < 10
        {
            // 0x30 + 0x0 == '0'
            // 0x30 + 0x1 == '1'
            // etc.
            uart_putc((0x30 + drepr) as u8 as char);
        } else
        {
            uart_putc('d');
        }
    }
    return 0;
}

/*
impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
*/

/*
 * Pulling from UART 16550 crate for formatting. 
 * Actually seems like that crate didn't have much real formatting going on...
 */
struct UartWriter
{
    buffer: &'static str,
}

/*
use core::fmt;

impl fmt::Write for UartWriter
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart_puts(s);
        return Ok(());
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    let mut uwriter: UartWriter = UartWriter {buffer: ""};
    uwriter.write_fmt(args);
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! uart_println {
    () => ($crate::uart_print!("\n"));
    ($fmt:expr) => ($crate::uart_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::uart_print!(concat!($fmt, "\n"), $($arg)*));
}
*/

/* Global printer with formatting */
use spin::Mutex;
use lazy_static::lazy_static;
// use core::fmt::Write;

pub struct UARTWriter {}

impl core::fmt::Write for UARTWriter
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result 
    {
        uart_puts(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref UARTWRITER: Mutex<UARTWriter> = Mutex::new(UARTWriter {});
}


#[doc(hidden)]
pub fn _uart_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    UARTWRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {
        _uart_print(format_args!($($arg)*));
    };
}