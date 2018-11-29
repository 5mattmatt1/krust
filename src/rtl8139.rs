use crate::serial_println;
const MAC05_OFFSET: u8 = 0x00;
const MAC05_SIZE: u8 = 0x06;
const MAR07_OFFSET: u8 = 0x08;
const MAR07_SIZE: u8 = 0x08;
const RBSTART_OFFSET: u32 = 0x30;
const RBSTART_SIZE: u8 = 0x04;
const CMD_OFFSET: u32 = 0x37;
const CMD_SIZE: u8 = 1;
const IMR_OFFSET: u32 = 0x3C;
const IMR_SIZE: u8 = 0x02;
const ISR_OFFSET: u32 = 0x3E;
const ISR_SIZE: u8 = 0x02;
const CONFIG_1_REG: u32 = 0x52;
const RCR_OFFSET: u32 = 0x44;

// Note:
// bus = 0
// slot = 3

pub unsafe fn power_on(ioaddr: u32)
{
    let power_on_address: u32 = ioaddr+CONFIG_1_REG;
    asm!("outl %eax, %dx" :: "{dx}"(power_on_address), "{eax}"(0x0) :: "volatile");
}

pub unsafe fn reset(ioaddr: u32) -> bool
{
    let cmd_offset: u32;
    let reset_address: u32 = ioaddr+CMD_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(reset_address), "{eax}"(0x10) :: "volatile");
    asm!("inl %dx, %eax" : "={eax}"(cmd_offset) : "{dx}"(reset_address) :: "volatile");
    // inb(ioaddr + CMD_OFFSET) & 0x10) != 0
    serial_println!("CMD_OFFSET: {}", cmd_offset);
    return ((cmd_offset) & 0x10) == 0;
}

pub unsafe fn init_rx(ioaddr: u32, rxbuffer_addr: &[u8; 1024])
{
    let rbstart_address: u32 = ioaddr + RBSTART_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(rbstart_address), "{eax}"(rxbuffer_addr) :: "volatile");
}

pub unsafe fn accept_tok(ioaddr: u32)
{
    let imroffset_address: u32 = ioaddr + IMR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(imroffset_address), "{eax}"(0x5) :: "volatile");
}

pub unsafe fn accept_rok(ioaddr: u32)
{
    let isroffset_address: u32 = ioaddr + ISR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(isroffset_address), "{eax}"(0x5) :: "volatile");
}

pub unsafe fn config_recieve_buffer_prom(ioaddr: u32)
{
    // outportl(ioaddr + 0x44, 0xf | (1 << 7));
    // Not configuring wrap bit
    let rcr_address: u32 = ioaddr + RCR_OFFSET;
    asm!("outl %eax, %dx" :: "{dx}"(rcr_address), "{eax}"(0xf) :: "volatile");
}

pub unsafe fn enable_recieve_and_transmit(ioaddr: u32)
{
    let rete_address: u32 = ioaddr + CMD_OFFSET;
    // outportb(ioaddr + CMD_OFFSET, 0x0C); // Sets the RE and TE bits high
    asm!("outl %eax, %dx" :: "{dx}"(rete_address), "{eax}"(0x10) :: "volatile");
}

pub fn acknowledge_rok()
{
    // outportw(ioaddr + ISR_OFFSET, 0x1);
}

pub unsafe fn setup_rtl8139(ioaddr: u32, rxbuffer_addr: &[u8; 1024]) -> bool
{
    let mut success: bool = true;
    power_on(ioaddr);
    success &= reset(ioaddr);
    init_rx(ioaddr, rxbuffer_addr);
    accept_tok(ioaddr);
    accept_rok(ioaddr);
    config_recieve_buffer_prom(ioaddr);
    enable_recieve_and_transmit(ioaddr);

    return success;
}