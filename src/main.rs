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

#![no_std]
#![no_main]
#![feature(asm)]
#![allow(dead_code)]
// Code should match style guides whenever physically possible
#![cfg_attr(feature ="strict", deny(private_in_public))]
#![cfg_attr(feature = "strict", deny(non_snake_case))]
#![cfg_attr(feature = "strict", deny(unused_parens))]
#![cfg_attr(feature = "strict", deny(unused_imports))]
#![cfg_attr(feature = "strict", deny(unused_unsafe))]
#![cfg_attr(feature = "strict", deny(unused_mut))]
#![cfg_attr(feature = "strict", deny(unused_variables))]
#![cfg_attr(feature = "strict", deny(unused_assignments))]

// #![feature(alloc)]
// #![feature(alloc_error_handler)]

// extern crate alloc;

pub mod bitmath;
pub mod vol;
pub mod gpio;
pub mod led;
#[macro_use]
pub mod uart;
pub mod mailbox;
// pub mod ferris;
// pub mod rpi_logo;
// pub mod exported_image;
// pub mod font;
// pub mod font0;
pub mod time;
// pub mod sdhci;
// pub mod mbr;
// pub mod fat;
pub mod dma;
pub mod panic;
// pub mod utils;
// pub mod memory;
pub mod mmu;

// #[global_allocator]
// static HEAPALLOCATOR: memory::Heap = memory::Heap
// {
//     kernel_page_table: [0; 4096],
//     user_page_table: [0; 4096],
// };

static kernel_page_table: [u8; 4096] = [0; 4096];
static user_page_table: [u8; 4096] = [0; 4096];

#[no_mangle]
fn kernel_entry() -> ! 
{
    use crate::uart::{Uart, mini::MiniUart};
    // HEAPALLOCATOR.init();
    // led::init();
    // led::blink(10, 1000000);
    MiniUart::init();
    uart_println!("Hello World!");
    mmu::init(&kernel_page_table, &user_page_table);
    // test_dma();
    // test_dma2();
    // test_sd();
    // test_mailbox();
    // echo everything back

    loop {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(kernel_entry);
