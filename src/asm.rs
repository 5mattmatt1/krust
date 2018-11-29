pub unsafe fn outl(eax: u32, dx: u32)
{
    asm!("outl %eax, %dx" :: "{dx}"(dx), "{eax}"(eax) :: "volatile");
}

pub unsafe fn inl(dx: u32) -> u32
{
    let ret: u32;
    asm!("inl %dx, %eax" : "={eax}"(ret) : "{dx}"(dx) :: "volatile");
    return ret;
}

/*
 * How many instructions does x86 assembly have?
 */