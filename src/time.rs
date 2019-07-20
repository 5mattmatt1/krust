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

const MMIO_BASE: u32 = 0x3F000000;
const SYSTMR_LO: u32 = MMIO_BASE + 0x00003004;
const SYSTMR_HI: u32 = MMIO_BASE + 0x00003008;

/// Wait N CPU cycles (ARM CPU only)
pub fn wait_cycles(mut n: u32)
{
    while n != 0
    {
        unsafe {
            asm!("nop");
        }
        n -= 1;
    }
}

/// Wait N microsec (ARM CPU only)
pub fn wait_msec(n: u64)
{
    let mut t: u64;
    let mut r: u64;
    let f: u64;
    unsafe
    {
        // get the current counter frequency
        asm!("mrs $0, cntfrq_el0" : "=r"(f));
        // read the current counter
        asm!("mrs $0, cntpct_el0" : "=r"(t));
    }
    // calculate expire value for counter
    t += ((f / 1000) * n) / 1000;
    loop
    {
        unsafe { asm!("mrs $0, cntpct_el0" : "=r"(r));}
        if r > t
        {
            break;
        }
    }
}