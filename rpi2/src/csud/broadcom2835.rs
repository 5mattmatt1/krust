// Should use crate::postman
// Maybe also have a module for postman
// That has a submodule for constants...

use volatile::Volatile;
const PERIPHERAL_ADDRESS: u32 = 0x3F000000;

const MAILBOX_READ: *mut u32 = (PERIPHERAL_ADDRESS + 0xB880) as *mut u32;
const TIMER_ADDR: *mut u64 = (PERIPHERAL_ADDRESS + 0x3004) as *mut u64;
const MAILBOX_FULL: u32 = 0x80000000;
const MAILBOX_EMPTY: u32 = 0x40000000;

fn micro_delay(u32 delay)
{
    unsafe { 
        u64 stop = *TIMER_ADDR + delay;

        while (*TIMER_ADDR < stop)
        {
            asm!("nop");
        }
    }
}

fn power_on_usb() -> bool
{
    unsafe 
    {
    let result: u32 = 0;
    // What is mailbox[8] ?
    // I feel like it is MAILBOX_WRITE
    // mailbox[6] must be MAILBOX_STATUS

    while *MAILBOX_STATUS & MAILBOX_FULL
    {
        asm!("nop");        
    }
    // Obviously 0x80 is the value to write
    // And channel 0 is to indicate power management
    // What does 0x80 mean exactly?
    *MAILBOX_WRITE = 0x80;
    while result & 0xF != 0
    {
        while *MAILBOX_READ & MAILBOX_EMPTY
        {
            asm!("nop");
        }
        result = *MAILBOX_READ;
    }

    return result == 0x80; 
    }
}