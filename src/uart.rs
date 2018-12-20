// TODO:
// Make a simple UART logger to increase
// my ability to debug
use crate::vol::{write32, read32};

/* Should be part off gpio.rs */
const GPFSEL1: u32 =    0x3F200004;
const GPSET0: u32 =     0x3F20001C;
const GPCLR0: u32 =     0x3F200028;
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
const AUX_MU_MSR_REG: u32 = 0x3F215058;
const AUX_MU_SCRATCH: u32 = 0x3F21505C;
const AUX_MU_CNTL_REG: u32 = 0x3F215060;
const AUX_MU_STAT_REG: u32 = 0x3F215064;
const AUX_MU_BAUD_REG: u32 = 0x3F215068;

// GPIO14 TXD0 and TXD1
// GPIO15 RXD0 and RXD1
// alt function 5 for uart1
// alt function 0 for uart0

// 7 << 12 // gpio14
// 2 << 12 // alt5

pub unsafe fn uart_setup()
{
    write32(AUX_ENABLES, 1);
    write32(AUX_MU_IER_REG, 0);
    write32(AUX_MU_CNTL_REG, 0);
    write32(AUX_MU_LCR_REG, 3);
    write32(AUX_MU_MCR_REG, 0);
    write32(AUX_MU_IER_REG, 0);
    write32(AUX_MU_IIR_REG, 0xC6);
    write32(AUX_MU_BAUD_REG, 270);

    let mut ra = read32(GPFSEL1);    
    ra &= !(7 << 12);
    ra |= (2 << 12); // alt5
    write32(GPFSEL1, ra);

    write32(GPPUD, 0);
    for i in 0..150
    {
        asm!("nop");
    }
    /* Usefulness? */
    write32(GPPUDCLK0, (1 << 14));
    for i in 0..150
    {
        asm!("nop");
    }
    write32(GPPUDCLK0, 0);

    write32(AUX_MU_CNTL_REG, 2);
}

pub unsafe fn uart_putc(c: char)
{
    while ((read32(AUX_MU_LSR_REG) & 0x20) == 0)
    {
        asm!("nop");
    }
    write32(AUX_MU_IO_REG, c as u32);
}

pub unsafe fn uart_getc() -> char
{
    while ((read32(AUX_MU_LSR_REG) & 0x01) == 0)
    {
        asm!("nop");
    }

    return read32(AUX_MU_IO_REG) as u8 as char;
}

// Fairly safe
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

pub unsafe fn uart_writeaddr(mut addr: usize)
{
    // Maximum u32 in hex: 7FFF,FFFF
    let mut delta: usize = 0;
    for i in (1..9)
    {
        // a % b == a & (b - 1)
        // Where b % 2 == 0
        delta = addr & ((16_usize.pow(i)) - 1);
        addr -= delta;
        // a << 2 == a * 4, but faster
        if delta >= 10
        {
            // 0x37 + 0xA == 'A'
            // 0x37 + 0xB == 'B'
            // etc.
            uart_putc((0x37 + (delta >> ((i-1) << 2))) as u8 as char);
        }
        else
        {
            // 0x30 + 0x0 == '0'
            // 0x30 + 0x1 == '1'
            // etc.
            uart_putc((0x30 + (delta >> ((i-1) << 2))) as u8 as char);
        }
    }
}