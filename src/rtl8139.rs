use crate::serial_println;
/* Gotten from https://wiki.osdev.org/RTL8139 */
const MAC05_OFFSET: u32 = 0x00;
const MAR07_OFFSET: u32 = 0x08;
const RBSTART_OFFSET: u32 = 0x30;
const CMD_OFFSET: u32 = 0x37;
const IMR_OFFSET: u32 = 0x3C;
const ISR_OFFSET: u32 = 0x3E;
const CONFIG_1_REG: u32 = 0x52;
const RCR_OFFSET: u32 = 0x44;
/* Gotten from drivers/net/ethernet/realtek */
const MAC0: u32 = 0x0;
const MAR0: u32 = 0x8;
const TxStatus0: u32 = 0x10;
const TxAddr0: u32 = 0x20;
const RxBuf: u32 = 0x30;
const ChipCmd: u32 = 0x37;
const RxBufPtr: u32 = 0x38;
const RxBufAddr: u32 = 0x3A;
const IntrMask: u32 = 0x3C;
const IntrStatus: u32 = 0x3E;
const TxConfig: u32 = 0x40;
const RxConfig: u32 = 0x44;
const Timer: u32 = 0x48;
const RxMissed: u32 = 0x4C;
const Cfg9346: u32 = 0x50;
const Config0: u32 = 0x51;
const Config1: u32 = 0x52;
const TimerInt: u32 = 0x54;
const MediaStatus: u32 = 0x58;
const Config3: u32 = 0x59;
const Config4: u32 = 0x5A;
const HltClk: u32 = 0x5B;
const MultiIntr: u32 = 0x5C;
const TxSummary: u32 = 0x60;
const BasicModeCtrl: u32 = 0x62;
const BasicModeStatus: u32 = 0x64;
const NWayAdvert: u32 = 0x66;
const NWayLPAR: u32 = 0x68;
const NWayExpansion: u32 = 0x6A;

/* Undocumented registers */
// I don't trust these 
// const FIFOTMS: u32 = 0x70;
// const CSCR: u32 = 0x74;
// const PARA78: u32 = 0x78;
// const FlashReg: u32 = 0xD4;
// const PARA7c: u32 = 0x7C;
// const Config5: u32 = 0xD8;

/* Sizes of registers */
const MAC05_SIZE: u8 = 0x06;
const MAR07_SIZE: u8 = 0x08;
const RBSTART_SIZE: u8 = 0x04;
const CMD_SIZE: u8 = 1;
const IMR_SIZE: u8 = 0x02;
const ISR_SIZE: u8 = 0x02;

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