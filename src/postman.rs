/* 
 * Lots of things changed between pi versions.
 * Going to try and use the property channel to get a framebuffer.
 */

use crate::vol::{write32, read32};
/* Raspberry Pi 2 */
const PERIPHERAL_ADDRESS: u32 = 0x3F000000;

// Mailbox addresses
const MAILBOX_READ: u32= (PERIPHERAL_ADDRESS + 0xB880) as u32;
const MAILBOX_STATUS: u32 = (PERIPHERAL_ADDRESS + 0xB898) as u32;
const MAILBOX_WRITE: u32 = (PERIPHERAL_ADDRESS + 0xB8A0) as u32;
const MAILBOX_FULL: u32 = 0x80000000;
const MAILBOX_EMPTY: u32 = 0x40000000;
const PROPERTY_CHANNEL: u8 = 8;

/*
 * Response codes given by 
 */
const RESPONSE_SUCCESS: u32 = 0x80000000;
const RESPONSE_ERROR: u32 = 0x80000001;

#[repr(u32)]
#[derive(PartialEq)]
enum PostmanErrorCodes
{
    MalformedStruct = 0x0,
    PostmanSuccess = 0x80000000,
    PostmanError = 0x80000001,
    UndefinedCode = 0xFFFFFFFF,
}

/*
 * Should I document certain codes for the property channel?
 */

unsafe fn us_mailbox_read(channel: u8) -> u32
{
    let mut status: u32;
    let mut result: u32;
    loop
    {
        loop 
        {
            status = read32(MAILBOX_STATUS);
            if status == MAILBOX_EMPTY
            {
                break;
            }
        }
        result = read32(MAILBOX_READ);
        if (result & 0xF) == channel as u32
        {
            break;
        }  
    }
    return result;
}

unsafe fn us_mailbox_send(mail: u32, channel: u8)
{
    /*
    let mut status: u32;
    loop
    {
        status = read32(MAILBOX_STATUS);
        if status != MAILBOX_FULL
        {
            break;
        }
    }
    */

    write32(MAILBOX_WRITE | channel as u32, mail | channel as u32);
}

fn rctoe(rc: u32) -> PostmanErrorCodes
{
    match rc
    {
        0 => return PostmanErrorCodes::MalformedStruct,
        RESPONSE_SUCCESS => return PostmanErrorCodes::PostmanSuccess,
        RESPONSE_ERROR => return PostmanErrorCodes::PostmanError,
        _ => return PostmanErrorCodes::UndefinedCode,
    }
}

/*
 * Just unsafe because of call to uart_puts
 */
unsafe fn debug_rc(debugMsg: &str, rc: PostmanErrorCodes)
{
    use crate::uart::uart_puts;
    
    uart_puts(debugMsg);
    if rc == PostmanErrorCodes::MalformedStruct
    {
        uart_puts("Malformed Struct.\n");
    } else if rc == PostmanErrorCodes::PostmanSuccess
    {
        uart_puts("Success!\n");
    } else if rc == PostmanErrorCodes::PostmanError
    {
        uart_puts("Error :(.\n");
    } else if rc == PostmanErrorCodes::UndefinedCode
    {
        uart_puts("Unknown error code.\n");
    }
}

#[repr(C, align(16))]
struct FbDescMsg
{
    buf_len: u32,
    response_code: u32,
    
    tag_screensize: u32,
    tag_ss_bitwidth: u32,
    tag_ss_padding: u32,
    tag_ss_width: u32,
    tag_ss_height: u32,

    tag_vscreensize: u32,
    tag_vss_bitwidth: u32,
    tag_vss_padding: u32,
    tag_vss_width: u32,
    tag_vss_height: u32,

    tag_bitdepth: u32,
    tag_bd_bitwidth: u32,
    tag_bd_padding: u32,
    tag_bd_bitdepth: u32,

    end_tag: u32, 
}

/* 
 * Could I template this with different sized arrays?
 * Possibly, but Rust is odd when it comes to arrays with size determined
 * at runtime, and I don't know if a heapless vector would have the same
 * format...
 * Should look into generic array crate. to at least have templates work.
 */
#[repr(C, align(16))]
struct FbInitMsg
{
    buf_len: u32,
    response_code: u32,
    
    tag_framebuffer_request: u32,
    tag_fbr_bitwidth: u32,
    tag_fbr_alignment: u32,
    base_ptr: u32, /* Actually framebuffer pointer */
    ptr_size: u32, /* Actually framebuffer size */

    end_tag: u32,
}

#[repr(C, align(16))]
struct GetPitchMsg
{
    buf_len: u32,
    response_code: u32,

    tag_getpitch_request: u32,
    tag_gp_bitwidth: u32,
    tag_gp_padding: u32,
    pitch: u32,

    end_tag: u32,
}

pub fn get_pitch() -> u32
{
    let mut get_pitch_msg: GetPitchMsg = GetPitchMsg
    {
        buf_len: 28,
        response_code: 0,
        
        tag_getpitch_request: 0x00040008,
        tag_gp_bitwidth: 4,
        tag_gp_padding: 4, /* Try changing to 8 */
        pitch: 0, /* 0 */

        end_tag: 0,
    };


    unsafe 
    {
        us_mailbox_send(&get_pitch_msg as *const GetPitchMsg as usize as u32, PROPERTY_CHANNEL);

        // For debugging purposes...
        debug_rc("Get pitch: ", rctoe(get_pitch_msg.response_code));
    }

    return get_pitch_msg.pitch;
}

/*
 * Dispmanx
 */
#[repr(C, align(16))]
struct GPUMsg
{
    mailbuffer: [u32; 256],
}

pub fn fb_initb()
{
    /* https://github.com/brianwiddas/pi-baremetal/blob/master/framebuffer.c */
    let mut mail: GPUMsg = GPUMsg {mailbuffer: [0; 256]};
    let mail_ptr: usize = (&mail as *const GPUMsg as usize) | 0xC000_0000; 
    // let mut mailbuffer: [u32; 256] = [0; 256];

    /* Get the display size */
    mail.mailbuffer[0] = 32; // Total size
    mail.mailbuffer[1] = 0; // Request
    mail.mailbuffer[2] = 0x40003; // Display size
    mail.mailbuffer[3] = 8; // Buffer size
    mail.mailbuffer[4] = 8; // Request size
    mail.mailbuffer[5] = 0; // Space for horz resolution
    mail.mailbuffer[6] = 0; // Space for vertical resolution
    mail.mailbuffer[7] = 0; // End tag

    unsafe
    {
        uart_puts("&mail[0]: ");
        uart_writeaddr(mail_ptr);
        us_mailbox_send(mail_ptr as *const u32 as usize as u32,
                        PROPERTY_CHANNEL);

        // us_mailbox_read(8);

        debug_rc("Get display size: ", 
                    rctoe(mail.mailbuffer[1]));
    }

    let mut fb_x: u32 = mail.mailbuffer[5];
    let mut fb_y: u32 = mail.mailbuffer[6];
    
    use crate::uart::{uart_puts, uart_putc, uart_writeaddr};

    /* If both fb_x and fb_y are both zero, assume we're running on the
	 * qemu Raspberry Pi emulation (which doesn't return a screen size
	 * at this point), and request a 640x480 screen
	 */
    if (fb_x == 0 && fb_y == 0)
    {
        uart_puts("In QEMU\n");
        fb_x = 640;
        fb_y = 480;
    } 
    if (fb_x == 640 && fb_y == 480)
    {
        uart_puts("Probs in QEMU\n");
    }

	mail.mailbuffer[0] = 84;		// Buffer size
    mail.mailbuffer[1] = 0; // Request
    mail.mailbuffer[2] = 0x00048003; // Tag id (set physical size)
    mail.mailbuffer[3] = 8;  // Value buffer size (bytes)
    mail.mailbuffer[4] = 8;  // Req. + value length (bytes)
    mail.mailbuffer[5] = fb_x;   // Horz resolution
    mail.mailbuffer[6] = fb_y;   // Vert resolution

    mail.mailbuffer[7] = 0x00048004; // Tag id (set virtual size)
    mail.mailbuffer[8] = 8;		// Value buffer size (bytes)
	mail.mailbuffer[9] = 8;		// Req. + value length (bytes)
	mail.mailbuffer[10] = fb_x;		// Horizontal resolution
	mail.mailbuffer[11] = fb_y;		// Vertical resolution

	mail.mailbuffer[12] = 0x00048005;	// Tag id (set depth)
	mail.mailbuffer[13] = 4;		// Value buffer size (bytes)
	mail.mailbuffer[14] = 4;		// Req. + value length (bytes)
	mail.mailbuffer[15] = 32;		// 32 bpp

	mail.mailbuffer[16] = 0x00040001;	// Tag id (allocate framebuffer)
	mail.mailbuffer[17] = 8;		// Value buffer size (bytes)
	mail.mailbuffer[18] = 4;		// Req. + value length (bytes)
	mail.mailbuffer[19] = 16;	// Alignment = 16
	mail.mailbuffer[20] = 0;		// Space for response
    // tag_fbr_bitwidth: 8,
    // tag_fbr_alignment: 16, /* Try changing to 8 */
    // base_ptr: 0, /* 0 */
    // ptr_size: 0,

	mail.mailbuffer[21] = 0;		// Terminating tag

    unsafe
    {
        us_mailbox_send(mail_ptr as u32,
                        PROPERTY_CHANNEL);
        // us_mailbox_read(8);

        debug_rc("Setup framebuffer: ", 
                    rctoe(mail.mailbuffer[1]));
    }


    let mut fb_ptr: u32 = mail.mailbuffer[19];

    /* Get the framebuffer pitch (bytes per line) */
	mail.mailbuffer[0] = 21;		// Total size
	mail.mailbuffer[1] = 0;		// Request
	mail.mailbuffer[2] = 0x40008;	// Display size
	mail.mailbuffer[3] = 4;		// Buffer size
	mail.mailbuffer[4] = 4;		// Request size
	mail.mailbuffer[5] = 0;		// Space for pitch
    mail.mailbuffer[6] = 0;     // End tag

    unsafe
    {
        us_mailbox_send(mail_ptr as u32,
                        PROPERTY_CHANNEL);
        // us_mailbox_read(8);

        debug_rc("Get pitch: ", 
                    rctoe(mail.mailbuffer[1]));
    }

    let pitch: u32 = mail.mailbuffer[5];

    // These aren't used by brianwiddas so maybe they are killing it...
    unsafe
    {
        uart_writeaddr(fb_ptr as usize);
    }
    fb_ptr &= 0x3FFFFFFF;
    // Notes:
    // OR'ing fb_ptr by 0x4000_0000 is something
    // From an old RPi 1 tutorial that I must not
    // have factored out when I made the switch from
    // channel 1 to channel 8.
    // Also OR'ing mail_ptr by 0xC000_0000 is to
    // Account for older models of the RPi 2.
    // fb_ptr |= 0x40000000;
    // fb_ptr &=!0xC0000000;
    unsafe
    {
        uart_writeaddr(fb_ptr as usize);
    }
    // test_gradient_render(fb_ptr, 0, 0, fb_x, fb_y, 24, fb_x);
    use crate::font0::FONT0;
    use crate::font::draw_string;
    
    unsafe
    {
        draw_string(FONT0, fb_ptr, fb_x, "Hello World!", 0, 0, 128, 96, 8, 16, 3);
    }
}

pub fn fb_init()
{
    // 80, /* The whole buffer is 80 bytes */
    // 0, /* Request response code is zero */
    // 0x00048003, 8, 0, 640, 480, /* This tag sets the screen size to 640 x 480 */
    // 0x00048004, 8, 0, 640, 480, /* This tag sets the virtual screen size to 640 x 480 */
    // 0x00048005, 4, 0, 24, /* This tag sets the depth to 24 bits */
    // 0,
    // 0, 0, 0
    /* Could add some args but for now this is good... */
    let buffer: FbDescMsg = FbDescMsg {
        buf_len: 80, 
        response_code: 0, 
        
        tag_screensize: 0x00048003, 
        tag_ss_bitwidth: 8, 
        tag_ss_padding: 8, 
        tag_ss_width: 640, 
        tag_ss_height: 480,
        
        tag_vscreensize: 0x00048004, 
        tag_vss_bitwidth: 8, 
        tag_vss_padding: 8, /* 0 */ 
        tag_vss_width: 640, 
        tag_vss_height: 480,
        
        tag_bitdepth: 0x00048005, 
        tag_bd_bitwidth: 4, 
        tag_bd_padding: 4, /* 0 */ 
        tag_bd_bitdepth: 24,
        
        end_tag: 0,
    };

    let mut get_fb: FbInitMsg = FbInitMsg {
        buf_len: 32,
        response_code: 0,
        
        tag_framebuffer_request: 0x00040001,
        tag_fbr_bitwidth: 8,
        tag_fbr_alignment: 16, /* Try changing to 8 */
        base_ptr: 0, /* 0 */
        ptr_size: 0,

        end_tag: 0,
    };

    use crate::uart::{uart_puts, uart_putc, uart_writeaddr};
    use crate::memory::{mem_v2p};

    unsafe
    {
        us_mailbox_send(&buffer as *const FbDescMsg as usize as u32,
                        PROPERTY_CHANNEL);
        debug_rc("Framebuffer Description Status: ", 
                rctoe(buffer.response_code));
        
        us_mailbox_send(&get_fb as *const FbInitMsg as usize as u32, PROPERTY_CHANNEL);

        let pitch = get_pitch();

        // let fb_len: u32 = get_fb.tag_fbr_padding2;
        // let fb = get_fb.tag_fbr_padding as *const [u32; 640*480*24];

        debug_rc("Framebuffer Init Status: ", 
                rctoe(get_fb.response_code));
        

        // debug_rc(release_buffer());
        get_fb.base_ptr |= 0x40000000;
        get_fb.base_ptr &=!0xC0000000;
        test_gradient_render(get_fb.base_ptr, 0, 0, 640, 480, 24, pitch);
        // use crate::font::{draw_font, draw_char, draw_string};
        // use crate::font0::FONT0;
        // let fontAddr = &FONT0[0] as *const u8 as usize;
        // draw_font(FONT0, get_fb.base_ptr, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, 'H', 0, 0, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, 'e', 8, 0, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, 'l', 16, 0, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, 'l', 24, 0, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, 'o', 32, 0, 128, 96, 8, 16, 3);
        // draw_char(FONT0, get_fb.base_ptr, '!', 40, 0, 128, 96, 8, 16, 3);
        // draw_string(FONT0, get_fb.base_ptr, "Hello World!", 0, 0, 128, 96, 8, 16, 3);
    }
}

/* Would help get rid of some of the passing of values along
 * Via functions and would give that nicce abstraction of a struct with implemented functions.
 */
struct Framebuffer
{
    addr: u32,
    x: u32,
    y: u32,
    phy_width: u32,
    phy_height: u32,
    virt_width: u32,
    virt_height: u32,
    bit_depth: u32,
    pitch: u32,
}

pub fn rgb24(r: u8, g: u8, b: u8) -> u32
{
    return (b as u32) << 16 | (g as u32) << 8 | r as u32; 
}

/*
 * Should move to test_gl.rs eventually
 */
pub fn test_gradient_render(addr: u32, 
            x: u32, y: u32, 
            phy_width: u32, phy_height: u32, 
            mut bit_depth: u32,
            mut pitch: u32)
{
    /*
     * TODO:
     * Pitch should be used instead of phy_width
     * due to the fact that the gpu won't always allocate
     * a framebuffer's rows continously.
     * However, I can't quite get the math working like I need to
     */
    // pitch >>= 2;
    bit_depth >>= 3;
    let mut r: i32 = 0x80;
    let mut g: i32 = 0x0;
    let mut b: i32 = 0x0;
    let mut r_step: i32 = 1;
    let mut g_step: i32 = 1;
    let mut color = rgb24(r as u8, g as u8, b as u8);
    for j in y..phy_height
    {
        // 320 is our magic number...
        for i in x..phy_width
        {
            unsafe 
            {
                draw_pixel(addr, i, j, pitch, bit_depth, color);
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
}

pub unsafe fn draw_pixel(addr: u32, x: u32, y: u32, pitch: u32, bit_depth: u32, color: u32)
{
    let pixel_offset: u32 = (x + (y * pitch)) * bit_depth;
    let pixel = (addr + pixel_offset) as *mut u32;
    *pixel = color;
}


/*
pub fn render(&mut self)
{
    let mut color = 0;
    // drawRow$:
    // ...
    // sub y, #1
    // ...
    // teq y, #0
    // bne drawRow$
    for j in 0..self.y
    {
        // drawPixel$:
        // ...
        // sub x, #1
        // teq x, #0
        // bne drawPixel$
        for i in 0..self.x
        {
            // add fbAddr, #2
            // addr_offset += 2;
            // strh colour, [fbAddr]
            // pixel = unsafe {framebuffer.offset(addr_offset) as *mut u32};
            // *pixel = color;
            self.draw_pixel(i, j, color);
        }
        // add colour, #1
        color += 1;
    }
}
	
pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32)
{
    let framebuffer = self.gpu_ptr as *const u32;
    let pixel_offset = (x + (y * self.phy_width)) * self.bit_depth;
    let pixel = unsafe {framebuffer.offset(pixel_offset as isize) as *mut u32};
    unsafe {*pixel = color};
}
	
pub fn draw_line(&mut self, mut x0: i32, x1: i32, mut y0: i32, y1: i32, color: u32)
{
    let delta_x: i32;
    let neg_delta_y: i32;
    let step_x: i32;
    let step_y: i32;
    let mut error: i32;
    if x1 > x0
    {
        delta_x = x1 - x0;
        step_x = 1;
    } else
    {
        delta_x = x1 - x0;
        step_x = -1;
    }
    
    if y1 > y0
    {
        neg_delta_y = -(y1 - y0);
        step_y = 1;
    } else
    {
        neg_delta_y = -(y1 - y0);
        step_y = -1;
    }
    
    // Bitshift by one is to simulate multiplication by 2
    error = 0;
    while x0 != x1 + step_x || y0 != y1 + step_y
    {
        self.draw_pixel(x0 as u32, y0 as u32, color);
        if (error << 1) > neg_delta_y
        {
            x0 += step_x;
            error += neg_delta_y;
        } else if (error << 1) < delta_x
        {
            y0 += step_y;
            error += delta_x;
        }
    }
}
*/

/*
 * TODO:
 * https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface#get-physical-display-widthheight
 * Implement all the cool codes that are contained within this github wiki page.
 */