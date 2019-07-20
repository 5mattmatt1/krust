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

use crate::gpio::MMIO_BASE;
use core::ptr::{write_volatile, read_volatile};

const DMA_BASE: u32 = (MMIO_BASE + 0x7000);
pub const DMA0_BASE: u32 = (MMIO_BASE + 0x7000);
// const DMA1_BASE: u32 = (MMIO_BASE + 0x7100);
// const DMA2_BASE: u32 = (MMIO_BASE + 0x7200);
// const DMAE_BASE: u32 = (MMIO_BASE + 0x7E00);
const DMA_ENABLE: *mut u32 = (DMA_BASE + 0xFF0) as *mut u32;
/// Need to find a good way to 
const DMA_CS: u32 = 0x0;
const DMA_CBA: u32 = 0x4;


/// 
#[repr(C)]
struct DmaChannelHeader
{
    /// Control and Status
    cs: u32,
    /// Control Block Address
    cba: u32,
    /// Transfer information
    ti: u32,
    /// Source address
    src: u32,
    /// Destination address
    dst: u32,
    /// Transfer length
    len: u32,
    /// 2D Stride
    stride: u32,
    /// Next CB Address
    ncba: u32,
    /// Debug
    dbg: u32,
}

/*
    /* DMA Control Block */
	DMA Transfer Information.
	bit26 : NO_WIDE_BURSTS : Don t Do wide writes as a 2 beat burst
	bit25:21 : WAITS : Add Wait Cycles
	bit20:16 : PERMAP : Peripheral Mapping
	bit15:12 : BURST_LENGTH : Burst Transfer Length
	bit11 : SRC_IGNORE : Ignore Reads
	bit10 : SRC_DREQ : Control Source Reads with DREQ
	bit 9 : SRC_WIDTH : Source Transfer Width
		1 = Use 128-bit source read width.
		0 = Use 32-bit source read width.
	bit 8 : SRC_INC : Source Address Increment
		1 = Source address increments after each read. The
		    address will increment by 4, if S_WIDTH=0 else by 32.
		0 = Source address does not change.
	bit 7 : DEST_IGNORE : Ignore Writes
	bit 6 : DEST_DREQ : Control Destination Writes with DREQ
	bit 5 : DEST_WIDTH : Destination Transfer Width
		1 = Use 128-bit destination write width.
		0 = Use 32-bit destination write width.
	bit 4 : DEST_INC : Destination Address Increment
		1 = Destination address increments after each write The address will increment by 4,
		    if DEST_WIDTH=0 else by 32.
		0 = Destination address does not change.
	bit 3 : WAIT_RESP : Wait for a Write Response
	bit 1 : TDMODE : 2D Mode
		1 = 2D mode interpret the TXFR_LEN register as
		    YLENGTH number of transfers each of XLENGTH, and
		    add the strides to the address after each transfer.
		0 = Linear mode interpret the TXFR register as a single
		    transfer of total length {YLENGTH ,XLENGTH}.
	bit 0 : INTEN : Interrupt Enable
 */

const DMA_TI_NO_WIDTH_BURSTS: u32 = 1 << 26;

const DMA_TI_WAITS_SHFT: u32 = 21;
const DMA_TI_PERMAP_SHFT: u32 = 16;
const DMA_TI_BURST_LEN_SHFT: u32 = 12;

const DMA_TI_SRC_IGN: u32 = 1 << 11;
const DMA_TI_SRC_DREQ: u32 = 1 << 10;
const DMA_TI_SRC_WIDTH128: u32 = 1 << 9;
const DMA_TI_SRC_WIDTH32: u32 = 0 << 9;
const DMA_TI_SRC_INC: u32 = 1 << 8;

const DMA_TI_DST_IGN: u32 = 1 << 7;
const DMA_TI_DST_DREQ: u32 = 1 << 6;
const DMA_TI_DST_WIDTH128: u32 = 1 << 5;
const DMA_TI_DST_WIDTH32: u32 = 0 << 5;
const DMA_TI_DST_INC: u32 = 1 << 4;

const DMA_TI_WAIT_RESP: u32 = 1 << 3;

const DMA_TI_2DMODE: u32 = 1 << 1;
const DMA_TI_LINEAR_MODE: u32 = 0 << 1;

const DMA_TI_INTEN: u32 = 1 << 0;

/// 
#[repr(C, align(32))]
pub struct DmaControlBlock
{
    pub ti: u32,
    pub src: u32,
    pub dst: u32,
    /// Length in BYTES not number of elements.
    pub len: u32,
    /// Should break up into dst_stride and src_stride
    pub stride: u32,
    pub ncba: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

const DMA0_CHANNEL_HEADER: *mut DmaChannelHeader = DMA0_BASE as *mut DmaChannelHeader;

pub struct DmaChannel
{
    addr: u32,
}

impl DmaChannel
{
    /// TODO: It looks like the reason that 2D mode was the improper stride values
    /// Because stride is actually two u16's.
    pub fn copy(&self, cb: &mut DmaControlBlock)
    {
        let cba_ptr: *mut u32 = (self.addr + DMA_CBA) as *mut u32;
        let cs_ptr: *mut u32 = (self.addr + DMA_CS) as *mut u32;
        
        unsafe 
        {
        write_volatile(DMA_ENABLE, 0x1);
        write_volatile(cba_ptr, cb as *mut DmaControlBlock as u32);
        write_volatile(cs_ptr, 0x1);

        let mut cs = read_volatile(cs_ptr);
        while cs & 0x2 == 0
        {
            cs = read_volatile(cs_ptr);
            asm!("nop");
        }
        write_volatile(cs_ptr, 0x2);
        }

    }
}

pub static DMACHANNEL0: DmaChannel = DmaChannel { addr: DMA0_BASE };