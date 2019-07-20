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
/// https://yannik520.github.io/sdio.html

use core::ptr::{read_volatile, write_volatile};

const SD_OK: i32 = 0;
const SD_TIMEOUT: i32 = -1;
const SD_ERROR: i32 = -2;

// use volatile::Volatile;

// sdhost:  mmc@7e202000
// sdhci: sdhci@7e300000

// External Mass Media Controller (SD card reader)
const MMIO_BASE: u32 = 0x3F00_0000;
const SDHCI_BASE: u32 = (MMIO_BASE + 0x0030_0000);
const SDHCI_ARG2: *mut u32          = (SDHCI_BASE + 0x0) as *mut u32;
const SDHCI_BLKSIZECNT: *mut u32        = (SDHCI_BASE + 0x4) as *mut u32;
// Breakdown of SDHCI_BLKSIZECNT
const SDHCI_BLKSIZE: *mut u16    = (SDHCI_BASE + 0x4) as *mut u16;
const SDHCI_BLK_COUNT: *mut u16     = (SDHCI_BASE + 0x6) as *mut u16;
const SDHCI_ARG1: *mut u32          = (SDHCI_BASE + 0x8) as *mut u32;
const SDHCI_CMDTM: *mut u32         = (SDHCI_BASE + 0xC) as *mut u32;
// Transfer mode
const SDHCI_TRANSFER_MODE: *mut u16         = (SDHCI_BASE + 0xC) as *mut u16;
const SDHCI_CMD: *mut u16                   = (SDHCI_BASE + 0xE) as *mut u16;

const SDHCI_RESP0: *mut u32         = (SDHCI_BASE + 0x10) as *mut u32;
const SDHCI_RESP1: *mut u32         = (SDHCI_BASE + 0x14) as *mut u32;
const SDHCI_RESP2: *mut u32         = (SDHCI_BASE + 0x18) as *mut u32;
const SDHCI_RESP3: *mut u32         = (SDHCI_BASE + 0x1C) as *mut u32;
const SDHCI_DATA: *mut u32          = (SDHCI_BASE + 0x20) as *mut u32;
const SDHCI_STATUS: *mut u32        = (SDHCI_BASE + 0x24) as *mut u32;
const SDHCI_CONTROL0: *mut u32        = (SDHCI_BASE + 0x28) as *mut u32;

/* Breakdown of SDHCI_CONTROL0 */
const SDHCI_HOST_CONTROL: *mut u8     = (SDHCI_BASE + 0x28) as *mut u8;
const SDHCI_POWER_CONTROL: *mut u8    = (SDHCI_BASE + 0x29) as *mut u8;
const SDHCI_BLOCK_GAP_CONTROL: *mut u8 = (SDHCI_BASE + 0x2A) as *mut u8;
const SDHCI_WAKE_UP_CONTROL: *mut u8 = (SDHCI_BASE + 0x2B) as *mut u8;

const SDHCI_CONTROL1: *mut u32        = (SDHCI_BASE + 0x2C) as *mut u32;

/* Breakdown of SDHCI_CONTROL1 */
const SDHCI_CLOCK_CONTROL: *mut u16   = (SDHCI_BASE + 0x2C) as *mut u16;
const SDHCI_TIMEOUT_CONTROL: *mut u8 = (SDHCI_BASE + 0x2E) as *mut u8;
const SDHCI_SOFTWARE_RESET: *mut u8 = (SDHCI_BASE + 0x2F) as *mut u8;

const SDHCI_INTERRUPT: *mut u32        = (SDHCI_BASE + 0x30) as *mut u32;
const SDHCI_INT_MASK: *mut u32        = (SDHCI_BASE + 0x34) as *mut u32;
const SDHCI_INT_EN: *mut u32        = (SDHCI_BASE + 0x38) as *mut u32;
// SDHCI_INTERRUPT -> SDHCI_INT_STATUS
// SDHCI_INT_MASK -> SDHCI_INT_ENABLE
// SDHCI_INT_EN -> SDHCI_SIG_ENABLE
const SDHCI_CONTROL2: *mut u32      = (SDHCI_BASE + 0x3E) as *mut u32;
const SDHCI_SLOTISR_VER: *mut u32          = (SDHCI_BASE + 0xFC) as *mut u32;

/* Command Flags */
const CMD_NEED_APP: u32  = 0x8000_0000;
const CMD_RSPNS_48: u32  = 0x0002_0000;
const CMD_ERRORS_MSK: u32 = 0xFFF9_C004;
const CMD_RCA_MSK: u32 = 0xFFFF_0000;

/* Commands */
const CMD_GO_IDLE: u32       =   0x0000_0000;
const CMD_ALL_SEND_CID: u32  =   0x0201_0000;
const CMD_SEND_REL_ADDR: u32 =   0x0302_0000;
const CMD_CARD_SELECT: u32   =   0x0703_0000;
const CMD_SEND_IF_COND: u32  =   0x0802_0000;
const CMD_STOP_TRANS: u32    =   0x0C03_0000;
const CMD_READ_SINGLE: u32   =   0x1122_0010;
const CMD_READ_MULTI: u32    =   0x1222_0032;
const CMD_SET_BLOCKCNT: u32  =   0x1702_0000;
const CMD_APP_CMD: u32       =   0x3700_0000; // CMD55
/* Needs app for whatever reason */
const CMD_SET_BUS_WIDTH: u32 =   (0x0602_0000 | CMD_NEED_APP);
const CMD_SEND_OP_COND: u32  =   (0x2902_0000 | CMD_NEED_APP);
const CMD_SEND_SCR: u32      =   (0x3322_0010 | CMD_NEED_APP);

/* Status Register settings */
const SR_READ_AVAILABLE: u32 = 0x800;
const SR_DAT_INHIBIT: u32    = 0x002;
const SR_CMD_INHIBIT: u32    = 0x001;
const SR_APP_CMD: u32        = 0x020;

/* Interrupt register settings */
const INT_DATA_TIMEOUT: u32  = 0x0010_0000;
const INT_CMD_TIMEOUT: u32   = 0x0001_0000;
const INT_READ_RDY: u32      = 0x0000_0020;
const INT_CMD_DONE: u32      = 0x0000_0001;

const INT_ERR_MASK: u32      = 0x017E_8000;

// Control register settings
/* Uses SPI for SD interfacing, interesting */
const C0_SPI_MODE_EN: u32    = 0x0010_0000;
/* HCTL? */
const C0_HCTL_HS_EN: u32     = 0x0000_0004;
const C0_HCTL_DWIDTH: u32    = 0x0000_0002;

/* SRST - Software Reset */
const C1_SRST_DATA: u32      = 0x0400_0000; 
const C1_SRST_CMD: u32       = 0x0200_0000;
const C1_SRST_HC: u32        = 0x0100_0000;
/* TOUNIT? */
// Dis and Max makes me think of Dispmax
// But that is part of VideoCoreOS...
const C1_TOUNIT_DIS: u32     = 0x000F_0000;
const C1_TOUNIT_MAX: u32     = 0x000E_0000;

/* CLK = Clock */
// Generate select
const C1_CLK_GENSEL: u32     = 0x0000_0020;
// Enable
const C1_CLK_EN: u32         = 0x0000_0004;
const C1_CLK_STABLE: u32     = 0x0000_0002;
const C1_CLK_INTLEN: u32     = 0x0000_0001;

// SLOTISR_VER values
const HOST_SPEC_NUM: u32         = 0x00FF_0000;
const HOST_SPEC_NUM_SHIFT: u32   = 0x10;
const HOST_SPEC_V3: u32          = 0x2;
const HOST_SPEC_V2: u32          = 0x1;
const HOST_SPEC_V1: u32          = 0x0;

// SCR - SD Card Configuration Register
const SCR_SD_BUS_WIDTH_4: u32    = 0x0000_0400;
const SCR_SUPP_SET_BLKCNT: u32   = 0x0200_0000;
// BZTSRC's value
const SCR_SUPP_CCS: u32          = 0x0000_0001;

// ACMD41 - Seems to coresspond to SEND_OP_COND a.k.a app command 41
const ACMD41_VOLTAGE: u32        = 0x00FF_8000;
const ACMD41_CMD_COMPLETE: u32   = 0x8000_0000;
/* Probably corresponds to SCR_SUPP_CCS */
// CCS - Card Capacity Status
const ACMD41_CMD_CCS: u32        = 0x4000_0000;
// HC? Maybe with HCTL?
// Possibly Hardware Control?

// References:
// https://lists.linaro.org/pipermail/boot-architecture/2011-August/000048.html
// C0FF_FF00 seems like an accurate high capcity card response for ocr...
// When am I supposed to get OCR?
const ACMD41_ARG_HC: u32         = 0x51FF_8000; // bztsrc
// const ACMD41_ARG_HC: u32 = 0xC0FF_FF00;

const BLOCK_SIZE: u32 = 0x100;

enum TransferMode
{
    DMA = 0x01,
    BlockCountEnable = 0x2,
    AutoCommand12 = 0x04,
    AutoCommand23 = 0x08,
    Read = 0x10,
    Multi = 0x20,
}

enum Command
{
    ResponseMask = 0x03,
}

enum WakeupControl
{
    OnInterrupt = 0x01,
    OnInsert = 0x02,
    OnRemove = 0x04,
}

pub trait SDIO
{
    fn init(&mut self) -> i32;
    fn readblock(&self, lba: usize) -> [u8; 512];
    fn status(mask: u32) -> i32;
    fn interrupt(mask: u32) -> i32;
    fn command(&self, mask: u32, arg: u32) -> (i32, u32);
    fn set_clock(&self, freq: u32) -> i32;
}

#[derive(Debug, Copy, Clone)]
pub struct RPISDIO
{
    scr: [u64; 3],
    ocr: u64,
    rca: u64, /* Relative Card Address */
    err: u64,
    hv: u64,
    ccs: u64,
}

/*
 * What's the difference between this one and bztsrc's?
 */

impl RPISDIO
{
    pub fn new() -> Self
    {
        return RPISDIO { scr: [0; 3], 
                         ocr: 0,  rca: 0,  err: 0, 
                         hv: 0, ccs: 0};
    }
   
    fn gpio_init(&mut self)
    {
        // Figure out, how to get the volatile wrapper into this
        use crate::gpio::*;
        
        use crate::time::*;
        unsafe
        {
        /* Is there a quicker way to setup all these GPIOs? */
        // Clearing some kind of GPIO...
        /* GPIO CD */
        // wand32 = write_volatile(val, read_volatile(val) & mod);
        write_volatile(GPFSEL4, read_volatile(GPFSEL4) & !(7 << (7 * 3))); /* Meaning? */
        write_volatile(GPPUD, 2); // Why 2?
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUDCLK1, 1 << 15); /* Meaning? */
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUD, 0);
        write_volatile(GPPUDCLK1, 0);
        write_volatile(GPHEN1, read_volatile(GPHEN1) | 1 << 15);

        /* GPIO CLK, GPIO_CMD */
        write_volatile(GPFSEL4, read_volatile(GPFSEL4) | (7 << (8 * 3)) | (7 << (9 * 3)));
        write_volatile(GPPUD, 2); // Why 2?
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUDCLK1, 1 << 16 | 1 << 17); /* Meaning? */
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUD, 0);
        write_volatile(GPPUDCLK1, 0);
        
        /* GPIO_DAT0, GPIO_DAT1, GPIO_DAT2, GPIO_DAT3 */
        write_volatile(GPFSEL5, read_volatile(GPFSEL5) | (7 << (0 * 3)) | (7 << (1 * 3)) |
                                                         (7 << (2 * 3)) | (7 << (3 * 3)));
        write_volatile(GPPUD, 2);
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUDCLK1, 1 << 18 | 1 << 19 | 1 << 20 | 1 << 21);
        wait_cycles(150); // Wait cycles
        write_volatile(GPPUD, 0);
        write_volatile(GPPUDCLK1, 0);
        self.hv = (read_volatile(SDHCI_SLOTISR_VER) & HOST_SPEC_NUM >> HOST_SPEC_NUM_SHIFT) as u64;
        uart_println!("{:X}", self.hv);
        }
    }
    
    fn reset_card(&self) -> i32
    {
        let mut cnt: u32 = 100_000;
        let mut sdhci_c1: u32;
        use crate::time::*;
        
        uart_println!("Attempting to reset card");
        
        unsafe
        {
        write_volatile(SDHCI_CONTROL0, 0);
        write_volatile(SDHCI_CONTROL1, read_volatile(SDHCI_CONTROL1) | C1_SRST_HC); // Software reset

        loop
        {
            wait_msec(10);
            sdhci_c1 = read_volatile(SDHCI_CONTROL1);
            cnt -= 1;
            if sdhci_c1 & C1_SRST_HC == 0 || cnt == 0
            {
                break;
            }
        }

        }

        if cnt == 0
        {
            return SD_ERROR;
        }
        return SD_OK;
    }
}

impl SDIO for RPISDIO
{
    /* Public functions */
    /// Initialize SDHCI to read SDHC card
    
    fn init(&mut self) -> i32
    {
        
        use crate::time::*;
        let mut stat: i32;
        let mut cmd_resp: (i32, u32);
        uart_println!("Initializing SDIO!");

        self.gpio_init();
        uart_println!("GPIO Init");
        stat = self.reset_card();
        if stat != SD_OK
        {
            uart_println!("Failed to reset card.");
            return stat;
        } else
        {
            uart_println!("Reset card");
        }

        wait_msec(10);
        // Not sure how to break this off
        unsafe {
            write_volatile(SDHCI_CONTROL1, read_volatile(SDHCI_CONTROL1) | C1_CLK_INTLEN | C1_TOUNIT_MAX);
        }
        wait_msec(10);
        // Set clock to setup frequency.
        stat = self.set_clock(100_000);
        if stat != SD_OK
        {
            uart_println!("Failed to set clock frequency to 400,000.");
            return stat;
        } else 
        {
            uart_println!("Set clock frequency to 400,000.");
        }
        
        unsafe {
            write_volatile(SDHCI_INT_EN, 0xFFFF_FFFF);
            write_volatile(SDHCI_INT_MASK, 0xFFF_FFFF);
        }

        self.rca = 0;

        cmd_resp = self.command(CMD_GO_IDLE, 0);
        if cmd_resp.0 != SD_OK
        {
            uart_println!("Failed to idle card.");
            return stat;
        }

        stat = self.acmd41_init();
        if stat != SD_OK
        {
            uart_println!("Failed to setup ACMD41.");
            return stat;
        } else
        {
            uart_println!("Setup ACMD41.");
        }

        // CID is some register...
        self.command(CMD_ALL_SEND_CID, 0);

        cmd_resp = self.command(CMD_SEND_REL_ADDR, 0);
        self.rca = cmd_resp.1 as u64;
        if cmd_resp.0 != SD_OK
        {
            uart_println!("Failed get relative card address.");
            return stat;
        }

        stat = self.set_clock(25_000_000);
        if stat != SD_OK
        {
            uart_println!("Failed to set clock freqeuncy to 25,000,000.");
            return stat;
        } else
        {
            uart_println!("Set clock frequency to 25,000,000");
        }

        cmd_resp = self.command(CMD_CARD_SELECT, self.rca as u32);
        if cmd_resp.0 != SD_OK
        {
            uart_println!("Failed to select card.");
            return stat;
        }

        if Self::status(SR_DAT_INHIBIT) != SD_OK
        {
            uart_println!("SDIO timeout.");
            return SD_TIMEOUT;
        }
        
        unsafe {
            write_volatile(SDHCI_BLKSIZECNT, read_volatile(SDHCI_BLKSIZECNT) | (1 << 16) | 8);
        }

        cmd_resp = self.command(CMD_SEND_SCR, 0);
        if cmd_resp.0 != SD_OK
        {
            uart_println!("Failed to send SCR.");
            return stat;
        } else
        {
            uart_println!("Sent SCR");
        }

        if Self::interrupt(INT_READ_RDY) != SD_OK
        {
            return SD_TIMEOUT;
        }

        uart_println!("SD Card is read ready.");
        // Current WIP
        self.read_scr();
        uart_println!("{:X}", self.scr[0]);
        uart_println!("{:X}", (self.scr[0] & SCR_SD_BUS_WIDTH_4 as u64));
        if self.scr[0] & SCR_SD_BUS_WIDTH_4 as u64 != 0
        {
            cmd_resp = self.command(CMD_SET_BUS_WIDTH, self.rca as u32 | 2);
            if cmd_resp.0 != SD_OK
            {
                return cmd_resp.0;
            }
            unsafe {
            write_volatile(SDHCI_CONTROL0, read_volatile(SDHCI_CONTROL0) | C0_HCTL_DWIDTH);
            }
            uart_println!("Set SD Bus width to 4");
        }

        uart_println!("SDHCI: supports ");
        if self.scr[0] & SCR_SUPP_SET_BLKCNT as u64 != 0
        {
            uart_println!("SET_BLKCNT ");
        }
        if self.ccs != 0
        {
            uart_println!("CCS ");
        }
        uart_println!("");

        self.scr[0] &= !SCR_SUPP_CCS as u64;
        self.scr[0] |= self.ccs as u64;
        return SD_OK;
    }

    /// Arguments:
    ///     * 'lba' (u32) - Logical block address
     
    fn readblock(&self, lba: usize) -> [u8; 512]
    {
        let mut buffer: [u8; 512] = [0; 512];
        let cmd_resp: (i32, u32);
        uart_println!("sd_readblock lba {:X}", lba);
        
        unsafe { write_volatile(SDHCI_BLKSIZECNT, (1 << 16) | 512) };
        cmd_resp = self.command(CMD_READ_SINGLE, lba as u32);
        if cmd_resp.0 != SD_OK
        {
            return buffer;
        }

        if Self::interrupt(INT_READ_RDY) != SD_OK
        {
            uart_println!("ERROR: Timeout waiting for ready to read");
            return buffer;
        }

        let mut data;
        for d in 0..128
        {
            unsafe { data = read_volatile(SDHCI_DATA) };
            /* Converts the u32 read from SDHCI data to 4 u32's */
            buffer[(d * 4) + 3] = ((data & 0xFF_00_00_00) >> 0x18) as u8;
            buffer[(d * 4) + 2] = ((data & 0x00_FF_00_00) >> 0x10) as u8;
            buffer[(d * 4) + 1] = ((data & 0x00_00_FF_00) >> 0x08) as u8;
            buffer[(d * 4) + 0] = ((data & 0x00_00_00_FF) >> 0x00) as u8;
        }

        return buffer;
    }

    /* Private functions */

    /// Arguments:
    ///     * 'freq' - Frequency of SD card I/O's clock.
    
    fn set_clock(&self, freq: u32) -> i32
    {
        
        use crate::time::*;
        uart_print!("Attempting to set SD clock");
        let mut d: u32; // Clock divisor 
        let c: u32 = 41_666_666/freq; // c?
        let mut x: u32 = c - 1; // x?
        let mut s: u32 = 32; // Clock shifter
        let mut h: u32 = 0;
        let mut cnt: u32 = 10_000;
        let mut sdhci_control1: u32;
        
        unsafe {
        let mut sdhci_status: u32 = read_volatile(SDHCI_STATUS);
        wait_msec(10);
        while sdhci_status & (SR_CMD_INHIBIT | SR_DAT_INHIBIT) != 0 && cnt != 0
        {
            sdhci_status = read_volatile(SDHCI_STATUS);
            wait_msec(10);
            cnt -= 1;
        }

        if cnt == 0
        {
            uart_println!("Timeout waiting for inhibit flag.");
            return SD_TIMEOUT;
        }

        write_volatile(SDHCI_CONTROL1, read_volatile(SDHCI_CONTROL1) & !C1_CLK_EN);
        wait_msec(10)
        }

        if x != 0 { s = 0; } else 
        {
            if x & 0xFFFF_0000 != 0 { x <<= 0x10; s -= 0x10; }
            if x & 0xFF00_0000 != 0 { x <<= 0x08; s -= 0x08; }
            if x & 0xF000_0000 != 0 { x <<= 0x04; s -= 0x04; }
            if x & 0xC000_0000 != 0 { x <<= 0x02; s -= 0x02; }
            if x & 0x8000_0000 != 0 { s -= 0x01; }
            if s > 0 { s -= 1; }
            if s > 7 { s = 7; }
        }

        if self.hv > HOST_SPEC_V2 as u64 { d = c; } else { d = 1 << s; }
        if d <= 2 { d = 2; } // s = 0; }
        if self.hv > HOST_SPEC_V2 as u64 { h = (d & 0x300) >> 2; }
        d = ((d & 0xFF) << 8) | h;

        unsafe {
        sdhci_control1 = read_volatile(SDHCI_CONTROL1);
        write_volatile(SDHCI_CONTROL1, (sdhci_control1 & 0xFFFF_003F) | d);
        wait_msec(10);
        write_volatile(SDHCI_CONTROL1, read_volatile(SDHCI_CONTROL1) | C1_CLK_EN);
        wait_msec(10);

        cnt = 10_000;
        sdhci_control1 = read_volatile(SDHCI_CONTROL1);
        while sdhci_control1 & C1_CLK_STABLE == 0 && cnt != 0
        {
            sdhci_control1 = read_volatile(SDHCI_CONTROL1);
            wait_msec(10);
            cnt -= 1;
        }
        }

        if cnt == 0
        {
            uart_println!("Failed to establish stable clock");
            return SD_TIMEOUT;
        }

        return SD_OK;
    }

    /// Brief:
    /// Wait for command or data ready.
    /// Arguments:
    ///     * 'mask' -
    
    fn status(mask: u32) -> i32
    {
        let mut cnt: u32 = 1000;
        let mut stat: u32 = !0;
        let mut int: u32 = 0;

        while stat != 0 && int == 0 && cnt != 0
        {
            unsafe {
                stat = read_volatile(SDHCI_STATUS) & mask;
                int = read_volatile(SDHCI_INTERRUPT) & INT_ERR_MASK; 
            }
            // wait_msec(1);
            cnt -= 1;
        }

        return if cnt == 0 || int != 0 {SD_ERROR} else {SD_OK};
    }

    /// Brief:
    /// Wait for interrupt
    /// Arguments:
    ///     * 'mask' -
    
    fn interrupt(mask: u32) -> i32
    {
        use crate::time::*;
        
        let int_mask: u32 = (mask as u32) | INT_ERR_MASK;
        let mut int: u32 = 0;
        let mut cnt: u32 = 100_000;
        unsafe {
        while int & int_mask == 0 && cnt != 0
        {
            wait_msec(1);
            int = read_volatile(SDHCI_INTERRUPT);
            cnt -= 1;
        }

        int = read_volatile(SDHCI_INTERRUPT);

        if cnt == 0 || int & INT_CMD_TIMEOUT != 0 || 
                       int & INT_DATA_TIMEOUT != 0
        {
            write_volatile(SDHCI_INTERRUPT, int);
            return SD_TIMEOUT;
        }

        if int & INT_ERR_MASK != 0
        {
            write_volatile(SDHCI_INTERRUPT, int);
            return SD_ERROR;
        }

        write_volatile(SDHCI_INTERRUPT, mask);
        }
        return SD_OK;
    }


    /// Brief:
    /// Send a command
    /// Parameters:
    ///     * 'code' - Which command to execute.
    ///     * 'arg' - Argument to pass along to the ARG0 register.
    /// TODO:
    /// What is the RCA?
    /// Change to an enum system for code to send.
    /// Need to figure out what this other damn thing is that
    /// is being sent out...
    
    fn command(&self, mut code: u32, arg: u32) -> (i32, u32)
    {
        use crate::time::*;
        let cmd_resp: (i32, u32);
        if code & CMD_NEED_APP != 0
        {
            if self.rca != 0
            {
                cmd_resp = self.command(CMD_APP_CMD | CMD_RSPNS_48, self.rca as u32); 
            } else
            {
                cmd_resp = self.command(CMD_APP_CMD, self.rca as u32);    
            }

            if cmd_resp.0 == SD_OK
            {
                uart_println!("Sent APP CMD");
            }

            if self.rca != 0 && cmd_resp.1 == 0
            {
                uart_println!("RCA is being a little bitch.");
                return (SD_ERROR, 0);
            }
            // In bztsrc's code, but wouldn't this make it no longer match
            // the later if statements and error?
            code &= !CMD_NEED_APP;
        }

        uart_println!("SDHCI: Sending command {:X} arg {:X}", code, arg);
        if Self::status(SR_CMD_INHIBIT) != SD_OK
        {
            uart_println!("Got invalid status");
            return (SD_TIMEOUT, 0);
        }

        /* Not sure why? Will have to see if bztsrc's code works without this. */
        unsafe {
            write_volatile(SDHCI_INTERRUPT, read_volatile(SDHCI_INTERRUPT));
        }
        // Why would you move an SDHCI_INTERRUPT into itself?
        unsafe {
            write_volatile(SDHCI_ARG1, arg);
            write_volatile(SDHCI_CMDTM, code);
        }

        if code == CMD_SEND_OP_COND & !CMD_NEED_APP
        {
            wait_msec(1000)
        } else if code == CMD_SEND_IF_COND || code == CMD_APP_CMD
        {
            wait_msec(100)
        }

        let mut err = Self::interrupt(INT_CMD_DONE);
        if err == SD_ERROR
        {
            uart_println!("Failed to send SDHCI command");
            return (err, 0);
        } else if err == SD_TIMEOUT
        {
            uart_println!("Sending SDHCI command timed out");
            return (err, 0);
        }

        let mut resp: u32;
        unsafe {
            resp = read_volatile(SDHCI_RESP0);
        }

        if code == CMD_GO_IDLE || code == CMD_APP_CMD
        {
            return (SD_OK, 0);
        } else if code == (CMD_APP_CMD | CMD_RSPNS_48)
        {
            uart_println!("RSPNS_48 RESP0: {:X}", resp);
            return (SD_OK, resp & SR_APP_CMD); 
        } else if code == CMD_SEND_OP_COND & !CMD_NEED_APP
        {
            uart_println!("SEND_OP_COND RESP0: {:X}", resp);
            return (SD_OK, resp);
        } else if code == CMD_SEND_IF_COND
        {
            if resp == arg
            {
                return (SD_OK, 0);
            }
            return (SD_ERROR, 0);
        } else if code == CMD_ALL_SEND_CID
        {
            unsafe {
                resp |= read_volatile(SDHCI_RESP3);
                resp |= read_volatile(SDHCI_RESP2);
                resp |= read_volatile(SDHCI_RESP1);
            }
            return (SD_OK, resp);
        } else if code == CMD_SEND_REL_ADDR
        {
            err = ((((resp & 0x1fff)) | ((resp & 0x2000) << 6)| ((resp & 0xC000) << 8)) & CMD_ERRORS_MSK) as i32;
            return (err, resp & CMD_RCA_MSK);
        }
        return (SD_OK, resp & CMD_ERRORS_MSK);
    }
}

impl RPISDIO
{
    pub fn dma_readblock(&self, lba: usize, buf: &mut [u32])
    {
        let cmd_resp: (i32, u32);
        uart_println!("sd_readblock lba {:X}", lba);
        
        unsafe { write_volatile(SDHCI_BLKSIZECNT, (1 << 16) | 512) };
        cmd_resp = self.command(CMD_READ_SINGLE, lba as u32);
        if cmd_resp.0 != SD_OK
        {
            return;
        }

        if Self::interrupt(INT_READ_RDY) != SD_OK
        {
            uart_println!("ERROR: Timeout waiting for ready to read");
            return;
        }


        // dma_alloc_writecombine
        // Use the VideoCore addresses for DMACopy'ing with SDHCI
        uart_println!("Starting DMACopy");
        crate::dma::DMACHANNEL0.copy(&mut crate::dma::DmaControlBlock {
            ti: 0x10 | 11 << 16 | 1 << 10,
            src: 0x7e300020, // SDHCI_DATA as u32,
            dst: &mut buf[0] as *mut _ as u32,
            len: 512,
            stride: 0,
            ncba: 0,
            reserved1: 0,
            reserved2: 0,
        });
    }
    
    // fn app_command(&self, code: u32, arg: u32) -> (i32, i32)
    // {
    //     return (SD_ERROR, -1);
    // }

    fn read_scr(&mut self) -> i32
    {
        use crate::time::*;
        
        let mut i = 0;
        let cnt = 100_000;
        unsafe {
        while i < 2 && cnt != 0
        {
            if read_volatile(SDHCI_STATUS) & SR_READ_AVAILABLE != 0
            {
                self.scr[i] = read_volatile(SDHCI_DATA) as u64;
                i += 1;
                uart_println!("SCR: {:X}", self.scr[i]);
            } else
            {
                wait_msec(1);
            }
        }
        }
        
        if i != 2
        {
            return SD_TIMEOUT;
        }

        return SD_OK;
    }

    
    fn acmd41_init(&mut self) -> i32
    {
        use crate::time::*;
        let mut cnt: u32 = 6;
        let mut cmd_resp: (i32, u32);
        cmd_resp = self.command(CMD_SEND_IF_COND, 0x0000_001AA); // Meaning?
        
        if cmd_resp.0 != SD_OK
        {
            uart_println!("Failed to send if condition command");
            return cmd_resp.0;
        } else
        {
            uart_println!("Sent send if condition command");
        }

        wait_msec(10);
        while cmd_resp.1 & ACMD41_CMD_COMPLETE == 0 && cnt != 0
        {
            cmd_resp = self.command(CMD_SEND_OP_COND, ACMD41_ARG_HC);
            
            if cmd_resp.0 == SD_ERROR
            {
                uart_println!("ACMD41 SEND_OP_COND caused error.");
                return cmd_resp.0;
            }
            wait_cycles(400);
            cnt -= 1;
        }

        if cmd_resp.1 & ACMD41_CMD_COMPLETE == 0 || cnt == 0
        {
            uart_println!("ACMD41 timed out");
            return SD_TIMEOUT;
        }

        if cmd_resp.1 & ACMD41_VOLTAGE == 0
        {
            uart_println!("Failed to setup voltage");
            return SD_ERROR;
        }

        if cmd_resp.1 & ACMD41_CMD_CCS != 0
        {
            self.ccs = SCR_SUPP_CCS as u64;
        } 

        return SD_OK;
    }

    pub fn dump_block(&self, lba: usize)
    {
        use crate::uart::mini::uart_putc_ascii;
        let buffer = self.readblock(lba);
        uart_println!("Read block");
        // dump line
        for line in 0..32
        {
            uart_print!("{:X} | ", line * 16);
            for column in 0..16
            {
                uart_print!("{:X} ", buffer[(line * 16) + column]);
            }
            uart_print!(" | ");
            for column in 0..16
            {
                uart_putc_ascii(buffer[(line * 16) + column] as char);
            }
            uart_println!("");
        }
        uart_println!("Dumped block");
    }
}