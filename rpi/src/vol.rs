/* Incorporate this into Krust */
use core::ptr::{read_volatile, write_volatile};

pub unsafe fn read16(addr: u32) -> u16 {
    read_volatile(addr as *const u16)
}

pub unsafe fn write16(addr: u32, value: u16) {
    write_volatile(addr as *mut u16, value);
}

pub unsafe fn read32(addr: u32) -> u32 {
    read_volatile(addr as *const u32)
}

pub unsafe fn write32(addr: u32, value: u32) {
    write_volatile(addr as *mut u32, value);
}

/*
 * Equivalent of |=
 */
pub unsafe fn wor32(addr: u32, value: u32)
{
    let rvalue: u32 = read32(addr);
    write32(addr, rvalue | value);
}