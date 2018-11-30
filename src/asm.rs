#[inline]
pub unsafe fn outb(dx: u32, al: u8)
{
    asm!("outb %al, %dx" :: "{dx}"(dx), "{al}"(al) :: "volatile");
}

#[inline]
pub unsafe fn inb(dx: u32) -> u8
{
    let al: u8;
    asm!("inb %dx, %al" : "={al}"(al) : "{dx}"(dx) :: "volatile");
    al
}

#[inline]
pub unsafe fn outw(dx: u32, ax: u16)
{
    asm!("outb %ax, %dx" :: "{dx}"(dx), "{ax}"(ax) :: "volatile");
}

#[inline]
pub unsafe fn inw(dx: u32) -> u16
{
    let ax: u16;
    asm!("inb %dx, %ax" : "={ax}"(ax) : "{dx}"(dx) :: "volatile");
    ax
}

#[inline]
pub unsafe fn outl(dx: u32, eax: u32)
{
    asm!("outl %eax, %dx" :: "{dx}"(dx), "{eax}"(eax) :: "volatile");
}

#[inline]
pub unsafe fn inl(dx: u32) -> u32
{
    let eax: u32;
    asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(dx) :: "volatile");
    eax
}

/*
 * How many instructions does x86 assembly have?
 */