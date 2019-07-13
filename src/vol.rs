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

pub unsafe fn wand32(addr: u32, value: u32)
{
    let rvalue: u32 = read32(addr);
    write32(addr, rvalue & value);
}
