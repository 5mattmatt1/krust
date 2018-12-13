fn mailbox_read(channel: u8, inchan: u32) -> u32
{
	let mailbox = MAILBOX_READ_ADDR as *const u32;
	// This is obviously neccessary for asm,
	// but rust would have a compile time check to make sure this doesn't overflow...
	// So might not really be neccessary
	// cmp r0, #15
	// movhi pc, lr
	if channel <= 0xF
	{
		// mail .req r2
		// ldr mail, [mailbox, #0]
		let mail = unsafe {mailbox.offset(0x0) as *mut u32};
		// inchan .req r3
		// and inchan,mail,#0b1111
		// teq inchan channel
		// .unreq inchan
		// bne rightmail$
		if (mail & 0b1111) == channel
		{
			// rightmail $
			let status = unsafe {mailbox.offset(0x18) as *mut u32};
			while (*status & 0x40000000) != 0 {}
			// and r0, mail, #0xfffffff0
			return mail & 0xfffffff0;
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

#repr(C)
struct FrambufferInfo
{
	phy_width: u32,
	phy_height: u32,
	virt_width: u32,
	virt_height: u32,
	gpu_pitch: u32,
	bit_depth: u32,
	x: u32,
	y: u32,
	gpu_ptr: u32,
	gpu_size: u32,
}

impl FrambufferInfo
{
	// True types:
	// width: u12
	// height: u12
	// bitDepth: u5
	fn new(width: u16, height: u16, bitDepth: u8) -> Self
	{
		if width < 4096 && height < 4096 && bitDepth < 32
		{
			let mut fbInfo = FrambufferInfo { 
				width as u32, 
				height as u32, 
				width as u32,
				height as u32,
				0,
				bit_depth as u32,
				0,
				0,
				0,
				0
			}
			mailbox_write(1, (&fbInfo as u32) + 0x40000000);
			if mailbox_read == 0
			{
				return fbInfo;
			}
		}
	}
	
	// More like render gradient...
	fn render(&mut self)
	{
		let y = 768;
		let x = 1024
		let mut color = 0;
		let mut addr_offset = 0;
		// drawRow$:
		// ...
		// sub y, #1
		// ...
		// teq y, #0
		// bne drawRow$
		for j in 0..y
		{
			// drawPixel$:
			// ...
			// sub x, #1
			// teq x, #0
			// bne drawPixel$
			for i in 0..x
			{
				// add fbAddr, #2
				// addr_offset += 2;
				// strh colour, [fbAddr]
				// pixel = unsafe {framebuffer.offset(addr_offset) as *mut u32};
				// *pixel = color;
				self.draw_pixel(x, y, color);
			}
			// add colour, #1
			color += 1;
		}
	}
	
	fn draw_pixel(x: u32, y: u32, color: u32)
	{
		let framebuffer = self.gpu_ptr as *const u32;
		let pixel_offset = (x + (y * self.phy_width)) * self.bit_depth;
		pixel = unsafe {framebuffer.offset(pixel_offset) as *mut u32};
		*pixel = color;
	}
	
	fn draw_line(x0: u32, x1: u32, y0: u32, y1: u32)
	{
		let dX: i32;
		let dYn: i32;
		let stepX: i32;
		let mut error: i32;
		if x1 > x0
		{
			dX = x1 - x0;
			stepX = 1;
		} else
		{
			dX = x1 - x0;
			stepX = -1;
		}
		
		if y1 > y0
		{
			dYn = -(y1 - y0);
			stepY = 1;
		} else
		{
			dYn = -(y1 - y0);
			stepY = -1;
		}
		
		// Bitshift by one is to simulate multiplication by 2
		error = 0;
		while x0 != x1 + stepX || y0 != y1 + stepY
		{
			setPixel(x0, y0);
			if (error << 1) > dYn
			{
				x0 += stepX;
				error -= dY;
			} else if (error << 1) < dX
			{
				y0 += stepY;
				error += dX;
			}
		}
		}
	}
}

// Needs to be moved to random.rs
// x acts as the seed
fn rand(x: u32)
{
	let a = 0xEF00;
	let b = (a + 1) % 4;
	let c = 0xFE01;
	return a * (x ** 2) + b * (x) + c;
}

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