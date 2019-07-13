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
#![feature(alloc)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod vol;
pub mod gpio;
#[macro_use]
pub mod uart;
pub mod postman;
pub mod font;
pub mod font0;
pub mod time;
pub mod sdhci;
pub mod mbr;
pub mod fat;
pub mod memory;

use core::panic::PanicInfo;

#[global_allocator]
static HEAPALLOCATOR: memory::Heap = memory::Heap;

// extern "C" {
//     static _end: char;
// }

fn test_gpu()
{
    use crate::postman::fb_initb;
    fb_initb();
}

/// Add this setting in QEMU for an SD card.
/// -drive file=test.dd,if=sd,format=raw
#[no_mangle]
fn test_sd() -> sdhci::RPISDIO
{
    use crate::mbr::*;
    use crate::uart::*;
    // let err: i32;
    // let sdio: sd::RPISDIO;
    use crate::sdhci::SDIO;
    let mut sdio = sdhci::RPISDIO::new();
    // let mbr: usize;
    // unsafe {
    // mbr = &_end as *const char as usize;
    // }
    sdio.init();
    // uart_puts("SDIO Init!\n");
    let mbr: MasterBoostRecord = MasterBoostRecord::new(sdio);
    // mbr.dump();
    let block0 = sdio.readblock(mbr.mpt.part0.start_sector as usize);
    let bpb = fat::BiosParameterBlock::new(block0);
    let ebpb = fat::ExtBiosParameterBlock::new(block0);
    // bpb.dump();
    // ebpb.dump();
    let mut fat_root_lba = (ebpb.num_sec_pfat as u32 * bpb.num_fat as u32) + bpb.num_rsec as u32;
    fat_root_lba += mbr.mpt.part0.start_sector;
    let fat_table_lba = mbr.mpt.part0.start_sector + bpb.num_rsec as u32;
    let fat_root = sdio.readblock(fat_root_lba as usize);
    // fat::DirEnt::new(&fat_root[..]).dump();
    // fat::DirEnt::new(&fat_root).dump();
    let mut vec = alloc::vec::Vec::new();
    vec.push(0);
    uart_print!("Allocation successful!");
    return sdio;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::uart::*;
    uart_print!("Kernel Panic! {}\n", info);
    loop {}
}

#[no_mangle]
fn kernel_entry() -> ! 
{
    unsafe 
    {
        use crate::uart::*;
        uart_setup();
        uart_puts("Hello World!\n");
        test_sd();
        // test_gpu();
        // echo everything back
        loop {
            asm!("nop");
        }
    }
}

raspi3_boot::entry!(kernel_entry);
