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

use super::Uart;
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

pub struct MiniUart;

impl Uart for MiniUart
{
    fn init()
    {
        unsafe 
        {
            wor32(AUX_ENABLES, 1);
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
    }

    fn putc(c: char)
    {
        unsafe {
            while (read32(AUX_MU_LSR_REG) & 0x20) == 0
            {
                asm!("nop"); 
            }
            write32(AUX_MU_IO_REG, c as u32);
        }
    }

    fn getc() -> char
    {
        unsafe {
            while (read32(AUX_MU_LSR_REG) & 0x01) == 0
            {
                asm!("nop");
            }
            return read32(AUX_MU_IO_REG) as u8 as char;
        }
    }
}

pub fn uart_putc_ascii(mut c: char)
{
    if (c as u8) < 0x20 || (c as u8) > 0x7F
    {
        c = '.';
    }
    MiniUart::putc(c);
}

/* Global printer with formatting */
use spin::Mutex;
use lazy_static::lazy_static;

impl core::fmt::Write for MiniUart
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result 
    {
        Self::puts(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref UARTWRITER: Mutex<MiniUart> = Mutex::new(MiniUart {});
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    UARTWRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::mini::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! uart_println {
    () => (uart_print!("\n"));
    ($fmt:expr) => (uart_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (uart_print!(concat!($fmt, "\n"), $($arg)*));
}