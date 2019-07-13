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

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct Heap;
// {
//     kernel_page_table: [u8; 4096],
//     user_page_table: [u8; 4096],
// }

impl Heap
{
    pub unsafe fn init()
    {
        // let heap = Heap {
        //     kernel_page_table: [0; 4096],
        //     user_page_table: [0; 4096]
        // };
        /* Thank you bztsrc */
        let mmfr: u64;
        asm!("mrs $0, id_aa64mmfr0_el1" : "=r"(mmfr));
        // asm!("msr mair_el1, $0" : "=r"());
        // asm!("msr tcr_el1, $0; isb" :"=r"());
        // tell the MMU where our translation tables are.
        // Userspace
        // asm!("msr ttbr0_el1, $0" : "=r"());
        // Kernelspace
        // asm!("msr ttbr1_el1, $0" : "=r"());

        // Finally, toggle some bits in system control register to enable page translation
        // asm!("dsb ish; isb; mrs $0, sctlr_el1" : "=r"());
        // asm!("msr sctlr_el1, $0; isb" : "=r"());
        // return heap;
    }
}

unsafe impl GlobalAlloc for Heap
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {

    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

// use spin::Mutex;
// use lazy_static::lazy_static;
// lazy_static!
// {
// }