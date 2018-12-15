// Based off tutorial from:
// https://www.cl.cam.ac.uk/projects/raspberrypi/tutorials/os/screen01.html

/* Raspberry Pi 1/Zero/Zero W */
// const PERIPHERAL_ADDRESS: u32 = 0x20000000;

/* Raspberry Pi 2 */
const PERIPHERAL_ADDRESS: u32 = 0x3F000000;

// Note: 
// Pine64 GPU is a Mali-400 MP2

// Mailbox addresses
// const MAILBOX_BASE_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB880;
// const MAILBOX_READ_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB880;
// const MAILBOX_POLL_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB890;
// const MAILBOX_SENDER_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB894;
// const MAILBOX_STATUS_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB898;
// const MAILBOX_CONFIG_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB89C;
// const MAILBOX_WRITE_ADDR: u32 = PERIPHERAL_ADDRESS + 0xB8A0;
const MAILBOX_READ: *mut u32 = (PERIPHERAL_ADDRESS + 0xB880) as *mut u32;
const MAILBOX_STATUS: *mut u32 = (PERIPHERAL_ADDRESS + 0xB898) as *mut u32;
const MAILBOX_WRITE: *mut u32 = (PERIPHERAL_ADDRESS + 0xB8A0) as *mut u32;
// Should be an enum
// const HIGH_COLOR_BIT_DEPTH: u32 = 16;
// const TRUE_COLOR_BIT_DEPTH: u32 = 24;
// const RGBA32_BIT_DEPTH: u32 = 32;

#[repr(C, align(4))]
pub struct FrameBufferInfo
{
    phy_width: u32,
    phy_height: u32,
    virt_width: u32,
    virt_height: u32,
    gpu_pitch: u32,
    bit_depth: u32, /* Could benefit from an enum */
    x: u32,
    y: u32,
    gpu_ptr: u32,
    gpu_size: u32,
}

use crate::gpio::turn_off_led;

// name .req register name
// Creates an alias for register name called name
fn write_to_mailbox(channel: u32, mut value: u32)
{
    /*
    tst r0, #0b1111
    movne pc, lr
    cmp r1, #15
    movhi pc, lr
    */
    // let mailbox = MAILBOX_BASE_ADDR as *const u32;
    if ((value & 0xF) == 0) && (channel < 0xF)
    {
        /*
        channel .req r1
        value .req r2
        mov value, r0
        push {lr}
        bl GetMailboxBase
        mailbox .req r0
        */
		// Maybe use MAILBOX_STATUS_ADDR as *mut u32
		// Or hell maybe even make it known that MAILBOX_STATUS_ADDR
		// Is a const pointer from the beginning, idk
		// And then I lose the unsafe code here?
        // let status = unsafe { mailbox.offset(0x18) as *mut u32};
        // Wait until a valid status
        /*
        wait1$:
        status .req r3
        ldr status, [mailbox, #0x18]
        tst status, #0x0x80000000
        .unreq status
        bne wait1$
        */
        while unsafe{((*MAILBOX_STATUS) & 0x80000000) != 0}
		{
			unsafe { asm!("nop") };
		}
        /*
        add value, channel
        .unreq channel
        */
        value |= channel;
        // str value, [mailbox, #0x20]
        // .unreq value
        // .unreq mailbox
        // pop {pc}
        // let write = unsafe { mailbox.offset(0x20) as *mut u32};
        unsafe { *MAILBOX_WRITE = value as u32 };
    } else { 
		/* Panic??? */
		// if (value + channel) != (value | channel)
		// {
		turn_off_led();
		// } 
	}
}

fn read_from_mailbox(channel: u8) -> u32
{
	// let mailbox = MAILBOX_READ_ADDR as *const u32;
	// This is obviously neccessary for asm,
	// but rust would have a compile time check to make sure this doesn't overflow...
	// So might not really be neccessary
	// cmp r0, #15
	// movhi pc, lr
	if channel <= 0xF
	{
		// mail .req r2
		// ldr mail, [mailbox, #0]
		// let mail = unsafe {mailbox as *mut u32};
		// inchan .req r3
		// and inchan,mail,#0b1111
		// teq inchan channel
		// .unreq inchan
		// bne rightmail$
		if unsafe {(*MAILBOX_READ & 0b1111) == channel as u32}
		{
			// rightmail $
			// let status = unsafe {mailbox.offset(0x18) as *mut u32};
			unsafe {
				while ((*MAILBOX_STATUS) & 0x40000000) != 0 { asm!("nop"); }
			// and r0, mail, #0xfffffff0
				return (*MAILBOX_READ) & 0xfffffff0;
			}
		} else 
		{
			// and r0, mail, #0xfffffff0
			return 0;
		}
		// .unreq mail
		// pop {pc}
	} else
	{ 
		/* Panic */ 
		return 0;
	}
}

impl FrameBufferInfo
{
	// True types:
	// width: u12
	// height: u12
	// bitDepth: u5
	pub fn new(width: u16, height: u16, bit_depth: u8) -> FrameBufferInfo
	{
		// if width < 4096 && height < 4096 && bitDepth < 32
		// {
        let fb_info = FrameBufferInfo { 
            phy_width: width as u32, 
            phy_height: height as u32, 
            virt_width: width as u32,
            virt_height: height as u32,
            gpu_pitch: 0,
            bit_depth: bit_depth as u32,
            x: 0,
            y: 0,
            gpu_ptr: 0,
            gpu_size: 0
        };

        let fb_addr = &fb_info as *const FrameBufferInfo as usize as u32;
		write_to_mailbox(1, fb_addr + 0x40000000);
		// I'm not sure if this is what is meant to happen
		if read_from_mailbox(1) != 0
		{
			// turn_off_led();
		}
		if fb_info.gpu_ptr == 0
		{
			turn_off_led();
		}
		return fb_info;
	}

	// More like render gradient...
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
}

// Needs to be moved to random.rs
// x acts as the seed
/*
fn rand(x: u32) -> u32
{
	let a = 0xEF00;
	let b = (a + 1) % 4;
	let c = 0xFE01;
	return (a * x.pow(2)) + (b * x) + c;
}
*/

/*
fn drawCharacter(character: char, x: u8, y: u8)
{
	// Need a font address
	let font = unsafe {FONT_ADDRESS.offset(character as u8 * 16)};
	for row in 0..15
	{
		let bits = readByte(charAddress + row)
		for bit in 0..7
		{
			if ((bits >> bit) & 0x1) == 0
			{
				setPixel(x + bit, y + row);
			}
		}
	}
}
*/

// Note for syscall.rs
// First syscall to implement will be uname so I can print out the name for FeOS/Krust
// Note if anything does not work:
// cd assembler
// git checkout master
// git stash
// git pull
// git stash pop
// Use code to merge in changes

// Time: 6:30 A.M. to 2:30 P.M. 12/13/2018