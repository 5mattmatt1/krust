const MAC05_OFFSET: u8 = 0x00;
const MAC05_SIZE: u8 = 0x06;
const MAR07_OFFSET: u8 = 0x08;
const MAR07_SIZE: u8 = 0x08;
const RBSTART_OFFSET: u8 = 0x30;
const RBSTART_SIZE: u8 = 0x04;
const CMD_OFFSET: u8 = 0x37;
const CMD_SIZE: u8 = 1;
const IMR_OFFSET: u8 = 0x3C;
const IMR_SIZE: u8 = 0x02;
const ISR_OFFSET: u8 = 0x3E;
const ISR_SIZE: u8 = 0x02;

const CONFIG_1_REG: u8 = 0x52;

// Note:
// bus = 0
// slot = 3

use x86_64::instructions::port::Port;
pub fn powerOn(port : Port)
{
    // outportb( ioaddr + CONFIG_1_REG, 0x0);
}

pub fn reset(port : Port) -> bool
{
    // outportb(ioaddr + CMD_OFFSET, 0x10);
    // outb(ioaddr)
    // inb(ioaddr + CMD_OFFSET) & 0x10) != 0
    return false;
}

pub fn init_rx(port : Port)
{
    // outportd(ioaddr + RBSTART_OFFSET, (uintptr_t)rx_buffer);
}

pub fn accept_tok(port : Port)
{
    // outportw(ioaddr + IMR_OFFSET, 0x0005);
}

pub fn accept_rok(port : Port)
{
    // outportw(ioaddr + ISR_OFFSET, 0x0005);
}

pub fn enable_recieve_and_transmit(port : Port)
{
    // outportb(ioaddr + CMD_OFFSET, 0x0C); // Sets the RE and TE bits high
}

pub fn acknowledge_rok(port : Port)
{
    // outportw(ioaddr + ISR_OFFSET, 0x1);
}