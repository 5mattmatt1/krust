use crate::gpio::*;
use crate::time::*;
use core::ptr::{read_volatile, write_volatile};

pub fn init()
{
    unsafe {
        let mut gpfsel1 = read_volatile(GPFSEL1);

        /* GPIO 16 = 001 - output */
        gpfsel1 &= !(7<<18);
        gpfsel1 |= 1<<18;
        /* GPIO 14 = 000 - input */
        gpfsel1 &= !(7<<12);

        /* Write back updated value */
        write_volatile(GPFSEL1, gpfsel1);

        /* Set up pull-up on GPIO14 */
        /* Enable pull-up control, then wait at least 150 cycles
        * The delay loop actually waits longer than that
        */
        write_volatile(GPPUD, 2);
        wait_msec(1000);

        /* Set the pull up/down clock for pin 14*/
        write_volatile(GPPUDCLK0, 1 << 14);
        write_volatile(GPPUDCLK1, 0);
        wait_msec(1000);

        /* Disable pull-up control and reset the clock registers */
        write_volatile(GPPUD, 0);
        write_volatile(GPPUDCLK0, 0);
        write_volatile(GPPUDCLK1, 0);
    }
}


pub fn blink(mut times: u32, delay: u64)
{
    while times != 0
    {
        unsafe { write_volatile(GPSET0, 1<<16) };
        wait_msec(delay);
        unsafe { write_volatile(GPCLR0, 1<<16) };
        times -= 1;
    }
}