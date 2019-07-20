// TODO: Add some mailbox code to blank the screen between tests
// in the proper manner.
#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate krust;

static mut FRAMEBUFFER: [u32; 307200] = [0; 307200];

pub fn rgb24(r: u8, g: u8, b: u8) -> u32
{
    return (b as u32) << 16 | (g as u32) << 8 | r as u32; 
}

pub fn rgba32(r: u8, g: u8, b: u8, a: u8) -> u32
{
    return (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
}

use krust::mailbox::vc::*;

/// DMA Clear screen
/// 
/// Much like clear_screen, but much like test_render_gradient2
/// uses dma to copy over personal FRAMEBUFFER into the given framebuffer
pub fn dma_clear_screen(fb: &Framebuffer)
{
    for j in 0..fb.phy_size.1
    {
        for i in 0..fb.phy_size.0
        {
            unsafe {
                FRAMEBUFFER[(i + (j * fb.phy_size.0)) as usize] = 0x0;
            }
        }
    }

    unsafe {
        krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
            ti: 0x330,
            src: &mut FRAMEBUFFER[0] as *mut _ as u32,
            dst: fb.ptr,
            len: (640 << 16) | 480,
            stride: fb.pitch >> 2,
            ncba: 0,
            reserved1: 0,
            reserved2: 0,
        });
    }
}

/// Modified version of test_gradient_render
/// that draws to a personal buffer first, and
/// then invokes dma to copy over all date from
/// into the framebuffer.
pub fn test_gradient_render2(fb: &Framebuffer)
{
    let mut r: i32 = 0x80;
    let mut g: i32 = 0x0;
    let b: i32 = 0x0;
    let mut r_step: i32 = 1;
    let mut g_step: i32 = 1;
    let mut color = rgb24(r as u8, g as u8, b as u8);
    
    for j in 0..fb.phy_size.1
    {
        for i in 0..fb.phy_size.0
        {
            unsafe {
                FRAMEBUFFER[(i + (j * fb.phy_size.0)) as usize] = color;
            }
            if (r + 1) == 256
            {
                r_step = -r_step;
            } else
            {
                r += r_step;
            }
            color = rgb24(r as u8, g as u8, b as u8);
        }
        if (g + g_step) == 256 || (g + g_step) == 0
        {
            g_step = -g_step;
        } else
        {
            g += g_step;
        }
    }    
    uart_println!("Drew to personal buffer.");
    uart_println!("Pitch: {}", fb.pitch);
    // Should work in 2D mode, but for some reason doesn't.
    // Maybe QEMU doesn't lay out memory properly?
    unsafe {
        krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
            ti: 0x330,
            src: &mut FRAMEBUFFER[0] as *mut _ as u32,
            dst: fb.ptr,
            len: (640 << 16) | 480,
            stride: fb.pitch >> 2,
            ncba: 0,
            reserved1: 0,
            reserved2: 0,
        });
    }
    uart_println!("DMACopy.");
}

/// Qemu isn't properly clearing the screen when I 
/// allocate/release Framebuffers and isn't blanking the screen
/// so I'm going to have to manually clear the screen
pub fn clear_screen(fb: &Framebuffer)
{
    for j in 0..fb.phy_size.1
    {
        for i in 0..fb.phy_size.0
        {
            fb.draw_pixel(i, j, 0x0);
        }
    }
}

/// Simple test that generates a framebuffer
/// using the VideoCore IV mailbox interfaces
/// and draws a red/orange/yellow gradient
/// background using a simple CPU copy.
pub fn test_gradient_render(fb: &Framebuffer)
{
    /*
     * TODO:
     * Pitch should be used instead of phy_width
     * due to the fact that the gpu won't always allocate
     * a framebuffer's rows continously.
     * However, I can't quite get the math working like I need to
     */
    let mut r: i32 = 0x80;
    let mut g: i32 = 0x0;
    let b: i32 = 0x0;
    let mut r_step: i32 = 1;
    let mut g_step: i32 = 1;
    let mut color = rgb24(r as u8, g as u8, b as u8);
    for j in 0..fb.phy_size.1
    {
        for i in 0..fb.phy_size.0
        {
            fb.draw_pixel(i, j, color);
            if (r + 1) == 256
            {
                r_step = -r_step;
            } else
            {
                r += r_step;
            }
            color = rgb24(r as u8, g as u8, b as u8);
        }
        if (g + g_step) == 256 || (g + g_step) == 0
        {
            g_step = -g_step;
        } else
        {
            g += g_step;
        }
    }
}

pub fn test_ferris(fb: &Framebuffer)
{
    krust::ferris::FERRIS.render(fb, 0, 0);
}

pub fn tear_down(fb: &Framebuffer)
{
    krust::time::wait_msec(1000000);
    // VCMailbox::blank_screen(1);
    clear_screen(fb);
    // VCMailbox::release_buffer();
    krust::time::wait_msec(1000000);
}

pub fn dma_tear_down(fb: &Framebuffer)
{
    krust::time::wait_msec(1000000);
    // VCMailbox::blank_screen(1);
    dma_clear_screen(fb);
    // VCMailbox::release_buffer();
    krust::time::wait_msec(1000000);
}

fn test_mailbox()
{
    let framebuffer = Framebuffer::new();
    uart_println!("test_gradient_render");
    test_gradient_render(&framebuffer);
    // framebuffer = Framebuffer::new();
    tear_down(&framebuffer);
    uart_println!("test_gradient_render2");
    
    test_gradient_render2(&framebuffer);
    dma_tear_down(&framebuffer);
    
    uart_println!("test_ferris");
    test_ferris(&framebuffer);
    dma_tear_down(&framebuffer);
}

// unsafe { asm!("BRK 0"); }
fn test_entry() -> ! 
{
    test_mailbox();
    loop 
    {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(test_entry);