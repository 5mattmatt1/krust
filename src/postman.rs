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
    let mut status: u32;
    loop
    {
        status = read32(MAILBOX_STATUS);
        if status != MAILBOX_FULL
        {
            break;
        }
    }

    write32(MAILBOX_WRITE, mail | channel as u32);
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
    tag_fbr_padding: u32, /* Actually framebuffer pointer */
    tag_fbr_alignment: u32,
    tag_fbr_padding2: u32, /* Actually framebuffer size */

    end_tag: u32,
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
        tag_ss_padding: 0, 
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
    /* [u32; 20] = 
        [80, 
        0, 
        0x00048003, 8, 0, 640, 480,
        0x00048004, 8, 0, 640, 480,
        0x00048005, 4, 0, 24,
        0,
        0, 0, 0]; */

    let get_fb: FbInitMsg = FbInitMsg {
        buf_len: 32,
        response_code: 0,
        
        tag_framebuffer_request: 0x00040001,
        tag_fbr_bitwidth: 8,
        tag_fbr_padding: 4, /* 0 */
        tag_fbr_alignment: 16,
        tag_fbr_padding2: 0,

        end_tag: 0,
    };
    
    /*
        [u32; 8] = 
        [32,
        0,
        0x00040001, 8, 0, 16, 0,
        0];
    */
    use crate::uart::{uart_puts, uart_putc, uart_writeaddr};
    use crate::memory::{mem_v2p};

    unsafe
    {
        us_mailbox_send(&buffer as *const FbDescMsg as usize as u32,
                        PROPERTY_CHANNEL);
        debug_rc("Framebuffer Description Status: ", 
                rctoe(buffer.response_code));
        
        us_mailbox_send(&get_fb as *const FbInitMsg as usize as u32, PROPERTY_CHANNEL);


        // let fb_len: u32 = get_fb.tag_fbr_padding2;
        let fb = get_fb.tag_fbr_padding as *const [u32; 640*480*24];

        debug_rc("Framebuffer Init Status: ", 
                rctoe(get_fb.response_code));
        
        /*
        if get_fb.response_code == 0x80000000
        {
            uart_puts("FB Success!\n");
        }
        */

        // debug_rc(release_buffer());
        render(get_fb.tag_fbr_padding, 0, 0, 640, 480, 24);
        /*
        // Size is odd, but we'll see...
        if (640*480*24) == get_fb.tag_fbr_padding2
        {
            uart_puts("Correct size\n");
        } else
        {
            uart_puts("Incorrect size\n");
        }
        */
    }
}

#[repr(C, align(16))]
struct BlankScreenMsg
{
    buf_len: u32,
    response_code: u32,

    tag_on: u32,
    tag_on_bitwidth: u32,
    tag_on_response: u32,
    tag_on_state: u32,

    end_tag: u32,
}

pub fn set_blank_screen(on: u32) -> bool
{
    let bls_msg: BlankScreenMsg = BlankScreenMsg {
        buf_len: 28,
        response_code: 0,
        
        tag_on: 0x00040002,
        tag_on_bitwidth: 4,
        tag_on_response: 0,
        tag_on_state: on,
        
        end_tag: 0,
    };
    
    unsafe 
    {
        us_mailbox_send(&bls_msg as *const BlankScreenMsg as usize as u32, PROPERTY_CHANNEL);
    }

    return bls_msg.response_code == RESPONSE_SUCCESS;
}

#[repr(C, align(16))]
struct ReleaseBufMsg
{
    buf_len: u32,
    response_code: u32,

    tag_release_buffer: u32, /* 0x00048001 */
    tag_rb_bitwidth: u32,

    end_tag: u32,
}


fn release_buffer() -> PostmanErrorCodes
{
    let rb_msg: ReleaseBufMsg = ReleaseBufMsg {
        buf_len: 20,
        response_code: 0,

        tag_release_buffer: 0x00048001,
        tag_rb_bitwidth: 0,

        end_tag: 0,
    };

    return rctoe(rb_msg.response_code);
}

/*
#[repr(C, align(16))]
struct ScreenSizeMsg
{
    buf_len: u32,
    response_code: u32,

    tag_screensize: u32,
    tag_ss_bitwidth: u32,
    tag_ss_response: u32,
    tag_ss_width: u32,
    tag_ss_height: u32,

    end_tag: u32,
}
*/

/*
 * Should move to framebuffer.rs eventually
 */
pub fn render(addr: u32, 
            x: u32, y: u32, 
            phy_width: u32, phy_height: u32, 
            bit_depth: u32)
{
    let mut color = 0;
    // drawRow$:
    // ...
    // sub y, #1
    // ...
    // teq y, #0
    // bne drawRow$
    for j in y..phy_height
    {
        // drawPixel$:
        // ...
        // sub x, #1
        // teq x, #0
        // bne drawPixel$
        for i in x..phy_width
        {
            // add fbAddr, #2
            // addr_offset += 2;
            // strh colour, [fbAddr]
            // pixel = unsafe {framebuffer.offset(addr_offset) as *mut u32};
            // *pixel = color;
            draw_pixel(addr, i, j, phy_width, bit_depth, color);
        }
        // add colour, #1
        color += 54;
    }
}

pub fn draw_pixel(addr: u32, x: u32, y: u32, phy_width: u32, bit_depth: u32, color: u32)
{
    let framebuffer = addr as *const u32;
    let pixel_offset = (x + (y * phy_width)) * bit_depth * 4;
    let pixel = unsafe {framebuffer.offset(pixel_offset as isize) as *mut u32};
    unsafe {*pixel = color};
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