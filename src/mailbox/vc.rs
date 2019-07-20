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

// https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#get-physical-display-widthheight
use super::Mailbox;
use super::Mail;

use core::ptr::{read_volatile, write_volatile};
use crate::gpio::MMIO_BASE;

/// Mailbox registers. We basically only support mailbox 0 & 1. We
/// deliver to the VC in mailbox 1, it delivers to us in mailbox 0. See
/// BCM2835-ARM-Peripherals.pdf section 1.3 for an explanation about
/// the placement of memory barriers.
const MBOX_BASE: u32 = MMIO_BASE + 0x0000_B880;
// const MBOX0: u32 = 0x00;
// const MBOX1: u32 = 0x20;
const MBOX_READ: *mut u32 = (MBOX_BASE + 0x00) as *mut u32;
// const MBOX_POLL: *mut u32 = (MBOX_BASE + 0x10) as *mut u32;
const MBOX_STATUS: *mut u32 = (MBOX_BASE + 0x18) as *mut u32;
const MBOX_WRITE: *mut u32 = (MBOX_BASE + 0x20) as *mut u32;

const MBOX_FULL: u32 = 0x8000_0000;
const MBOX_EMPTY: u32 = 0x4000_0000;

const PROPERTY_CHANNEL: u8 = 8;

// const GET_TAG: 0x0_0000;
// const TST_TAG: 0x0_4000;
// const SET_TAG: 0x0_8000;
/* Tags */
// VideoCore
const GET_FIRMWARE_VER_TAG: u32 = 0x00001;
// Hardware
const GET_BOARD_MODEL_TAG: u32 = 0x10001;
const GET_BOARD_REV_TAG: u32 = 0x10002;
const GET_BOARD_MAC_ADDR_TAG: u32 = 0x10003;
const GET_BOARD_SERIAL_TAG: u32 = 0x10004;
const GET_ARM_MEMORY_TAG: u32 = 0x10005;
const GET_VC_MEMORY_TAG: u32 = 0x10006;
const GET_CLOCK_TAG: u32 = 0x10007;
// Config
const GET_COMMAND_LINE_TAG: u32 = 0x50001;
// Shared resource management
const GET_DMA_CHANNELS_TAG: u32 = 0x60001;
// Power
const GET_POWER_STATE_TAG: u32 = 0x20001;
const SET_POWER_STATE_TAG: u32 = 0x28001;
const GET_TIMING_TAG: u32 = 0x20002;
// Clocks
const GET_CLOCK_STATE_TAG: u32 = 0x30001;
const SET_CLOCK_STATE_TAG: u32 = 0x38001;
const GET_CLOCK_RATE_TAG: u32 = 0x30002;
const SET_CLOCK_RATE_TAG: u32 = 0x38002;
const GET_MAX_CLOCK_RATE_TAG: u32 = 0x30004;
const GET_MIN_CLOCK_RATE_TAG: u32 = 0x30007;
const GET_TURBO_TAG: u32 = 0x30009;
const SET_TURBO_TAG: u32 = 0x38009;
// Voltage
const GET_VOLTAGE_TAG: u32 = 0x30003;
const SET_VOLTAGE_TAG: u32 = 0x38003;
const GET_MAX_VOLTAGE_TAG: u32 = 0x30005;
const GET_TEMPERATURE_TAG: u32 = 0x30006;
const GET_MIN_VOLTAGE_TAG: u32 = 0x30008;
const GET_MAX_TEMPERATUE_TAG: u32 = 0x3000A;
// 
const ALLOCATE_MEMORY_TAG: u32 = 0x3000C;
const LOCK_MEMORY_TAG: u32 = 0x3000D;
const UNLOCK_MEMORY_TAG: u32 = 0x3000E;
const RELEASE_MEMORY_TAG: u32 = 0x3000F;
const EXECUTE_CODE_TAG: u32 = 0x30010;
/// Get Dispmanx Resource mem handle
const GET_DISPMANX_HANDLE_TAG: u32 = 0x30014;
const GET_EDID_BLOCK_TAG: u32 = 0x30020;

// Framebuffer Tag's
const ALLOC_FRAMEBUFFER_TAG: u32 = 0x40001;
const RELEASE_FRAMEBUFFER_TAG: u32 = 0x48001;
const BLANK_SCREEN_TAG: u32 = 0x40002;
const GET_DISP_SIZE_TAG: u32 = 0x40003;
const TST_DISP_SIZE_TAG: u32 = 0x44003;
const SET_DISP_SIZE_TAG: u32 = 0x48003;
const GET_VIRT_SIZE_TAG: u32 = 0x40004;
const TST_VIRT_SIZE_TAG: u32 = 0x44004;
const SET_VIRT_SIZE_TAG: u32 = 0x48004;
const GET_BIT_DEPTH_TAG: u32 = 0x40005;
const TST_BIT_DEPTH_TAG: u32 = 0x44005;
const SET_BIT_DEPTH_TAG: u32 = 0x48005;
const GET_PIXEL_ORD_TAG: u32 = 0x40006;
const TST_PIXEL_ORD_TAG: u32 = 0x44006;
const SET_PIXEL_ORD_TAG: u32 = 0x48006;
const GET_ALPHA_MDE_TAG: u32 = 0x40007;
const TST_ALPHA_MDE_TAG: u32 = 0x44007;
const SET_ALPHA_MDE_TAG: u32 = 0x48007;
const GET_PITCH_TAG: u32 = 0x40008;
const GET_VIRT_OFST_TAG: u32 = 0x40009;
const TST_VIRT_OFST_TAG: u32 = 0x44009;
const SET_VIRT_OFST_TAG: u32 = 0x48009;
const GET_OVERSCAN_TAG: u32 = 0x4000A;
const TST_OVERSCAN_TAG: u32 = 0x4400A;
const SET_OVERSCAN_TAG: u32 = 0x4800A;
const GET_PALETTE_TAG: u32 = 0x4000B;
const TST_PALETTE_TAG: u32 = 0x4400B;
const SET_PALETTE_TAG: u32 = 0x4800B;
const SET_CURSOR_INFO_TAG: u32 = 0x8010;
const SET_CURSOR_STATE_TAG: u32 = 0x8011;

pub enum PowerDeviceId
{
    SdCard = 0x0,
    Uart0 = 0x1,
    Uart1 = 0x2,
    UsbHcd = 0x3,
    I2C0 = 0x4,
    I2C1 = 0x5,
    I2C2 = 0x6,
    SPI = 0x7,
    Ccp2tx = 0x8,
}

pub enum ClockId
{
    Reserved = 0x0,
    Emmc = 0x1,
    Uart = 0x2,
    Arm = 0x3,
    Core = 0x4,
    V3D = 0x5,
    H264 = 0x6,
    ISP = 0x7,
    Sdram = 0x8,
    Pixel = 0x9,
    Pwm = 0xA,
    Emmc2 = 0xC,
}

pub enum VoltageId
{
    Reserved = 0x0,
    Core = 0x1,
    SdramC = 0x2,
    SdramP = 0x3,
    SdramI = 0x4,
}

pub enum MemoryFlags
{
    /// can be resized to 0 at any time. Use for cached data
    Discardable = 1 << 0,
    /// normal allocating alias. Don't use from ARM
    Normal = 0 << 2,
    /// 0xC alias uncached
    Direct = 1 << 2,
    /// 0x8 alias. Non-allocating in L2 but coherent
    Coherent = 2 << 2,
    /// Allocating in L2
    // L1_NoAlloc = 
    /// initialise buffer to all zeros
    Zero = 1 << 4,
    /// don't initialise (default is initialise to all ones
    NoInit = 1 << 5,
    /// Likely to be locked for long periods of time
    HintPermalock = 1 << 6,
}

pub struct VCMailbox;

impl Mailbox for VCMailbox
{
    fn send(channel: u8, mail: Mail)
    {
        let mut status: u32;
        let mbox_ptr = (MBOX_WRITE as usize | channel as usize) as *mut u32;
        unsafe
        {
            loop
            {
                status = read_volatile(MBOX_STATUS);
                if status != MBOX_FULL
                {
                    break;
                }
            }

            write_volatile(mbox_ptr, mail | channel as u32);
        }
    }

    fn read(channel: u8) -> Mail
    {

        let mut status: u32;
        let mut result: Mail;
        unsafe {
            loop
            {
                loop 
                {
                    status = read_volatile(MBOX_STATUS);
                    if status == MBOX_EMPTY
                    {
                        break;
                    }
                }

                result = read_volatile(MBOX_READ);
                if (result & 0xF) == channel as u32
                {
                    break;
                }  
            }
        }
        return result;
    }
}

#[repr(C, align(16))]
pub struct VCMail<T>
{
    size: u32,
    request: u32,
    tag: u32,
    buffer_size: u32,
    request_size: u32,
    message: T,
    end: u32
}

#[repr(C)]
pub struct GetDisplaySize
{
    horz_res: u32,
    vert_res: u32,
}

#[repr(C)]
pub struct SetBitDepth
{
    bit_depth: u32,
}

#[repr(C)]
pub struct AllocateFramebuffer
{
    /// For some reason, the pointer to actual data
    /// is stored back into alignment here.
    alignment: u32,
    fb_size: u32,
}

/// Get bytes per line for framebuffer
#[repr(C)]
pub struct GetPitch
{
    pitch: u32,
}

/// Nothing 
pub struct ReleaseFramebuffer;

pub type GetDisplaySizeMail = VCMail<GetDisplaySize>;
pub type SetBitDepthMail = VCMail<SetBitDepth>;
pub type AllocateFramebufferMail = VCMail<AllocateFramebuffer>;
pub type ReleaseFramebufferMail = VCMail<ReleaseFramebuffer>;
pub type GetPitchMail = VCMail<GetPitch>;
pub type BlankScreenMail = VCMail<u32>;

impl VCMailbox
{
    // Simple function to simplify sending other types around
    fn vcsend<T>(mail: &mut VCMail<T>)
    {
        uart_println!("Sending mail through VC IV mailbox.");

        // Maybe needed?
        // 
        let ptr = mail as *mut VCMail<T> as u32;
        uart_println!("Pointer: 0x{:X}", ptr);
        Self::send(PROPERTY_CHANNEL, mail as *mut VCMail<T> as u32 | 0xC000_0000);
    }

    /// Simply controls whether the screen is blank or not.
    pub fn blank_screen(state: u32) -> u32
    {
        let mut mail: BlankScreenMail = BlankScreenMail {
            size: 24,
            request: 0,
            tag: BLANK_SCREEN_TAG,
            buffer_size: 4,
            request_size: 4,
            message: state,
            end: 0,
        };
        Self::vcsend(&mut mail);
        uart_println!("Blank Screen Response: 0x{:X}", mail.request);
        return mail.message;
    }

    /// Releases previously allocated framebuffer
    pub fn release_buffer()
    {
        let mut mail: ReleaseFramebufferMail = ReleaseFramebufferMail {
            size: 24,
            request: 0,
            tag: RELEASE_FRAMEBUFFER_TAG,
            buffer_size: 0,
            request_size: 0,
            message: ReleaseFramebuffer,
            end: 0,
        };
        Self::vcsend(&mut mail);
        uart_println!("Release buffer Response: 0x{:X}", mail.request);
    }

    fn get_display_size() -> (u32, u32)
    {
        let mut mail: GetDisplaySizeMail = GetDisplaySizeMail {
            size: 32,
            request: 0,
            tag: GET_DISP_SIZE_TAG,
            buffer_size: 8,
            request_size: 8,
            message: GetDisplaySize {
                horz_res: 0,
                vert_res: 0,
            },
            end: 0,
        };

        Self::vcsend(&mut mail);
        return (mail.message.horz_res, mail.message.vert_res);
    }

    fn set_bitdepth(bitdepth: u32)
    {
        let mut mail: SetBitDepthMail = SetBitDepthMail {
            size: 24,
            request: 0,
            tag: SET_BIT_DEPTH_TAG,
            buffer_size: 4,
            request_size: 4,
            message: SetBitDepth {
                bit_depth: bitdepth,
            },
            end: 0,
        };

        Self::vcsend(&mut mail);
    }

    fn get_pitch() -> u32
    {
        let mut mail: GetPitchMail = GetPitchMail{
            size: 21,
            request: 0,
            tag: GET_PITCH_TAG,
            buffer_size: 0,
            request_size: 4,
            message: GetPitch {
                pitch: 0,
            },
            end: 0,
        };

        Self::vcsend(&mut mail);
        return mail.message.pitch;
    }

    fn allocate_framebuffer(alignment: u32) -> u32
    {
        let mut mail: AllocateFramebufferMail = AllocateFramebufferMail {
            size: 32,
            request: 0,
            tag: ALLOC_FRAMEBUFFER_TAG,
            buffer_size: 8,
            request_size: 8,
            message: AllocateFramebuffer {
                alignment: alignment,
                fb_size: 0,
            },
            end: 0,
        };

        Self::vcsend(&mut mail);
        return mail.message.alignment;
    }
}

pub struct Framebuffer
{
    pub ptr: u32,
    pub phy_size: (u32, u32),
    pub bit_depth: u32,
    pub pitch: u32,
}

impl Framebuffer
{
    pub fn new() -> Self
    {
        let bit_depth: u32 = 32;
        let phy_size: (u32, u32) = VCMailbox::get_display_size();
        uart_println!("Display size: ({}, {})", phy_size.0, phy_size.1);
        VCMailbox::set_bitdepth(bit_depth);
        let ptr: u32 = VCMailbox::allocate_framebuffer(16);
        uart_println!("Framebuffer Address: 0x{:X}", ptr);
        let pitch: u32 = VCMailbox::get_pitch();
        // Why does this stuff have to be bit shifted?
        uart_println!("Pitch: {}", pitch);
        return Framebuffer {
            ptr: ptr & 0x3FFFFFFF,
            phy_size: phy_size,
            bit_depth: bit_depth >> 3,
            pitch: pitch >> 2,
        }
    }

    pub fn draw_pixel(&self, x: u32, y: u32, color: u32)
    {
        // See if this all works fine if I just remove the bit shifts and the bit depth...
        let pixel_offset: u32 = (x + (y * self.pitch)) * self.bit_depth;
        let pixel = (self.ptr + pixel_offset) as *mut u32;
        unsafe { *pixel = color; }
    }
}