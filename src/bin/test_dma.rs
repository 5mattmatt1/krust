#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate krust;

fn test_dma()
{
    let mut array0: [u32; 8] = [0xAC, 0xDC, 0xFF, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
    let mut array1: [u32; 8] = [0; 8];
    uart_println!("Start DMACopy");
    krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
        ti: 0x330,
        src: &mut array0[0] as *mut _ as u32,
        dst: &mut array1[0] as *mut _ as u32,
        len: 32,
        stride: 0,
        ncba: 0,
        reserved1: 0,
        reserved2: 0,
    });
    for i in 0..8
    {
        uart_println!("array1[{:X}] = {:X}", i, array1[i]);
    }
}

fn test_dma2()
{
    let mut array0: [u32; 8] = [0xAC, 0xDC, 0xFF, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
    let mut array1: [u32; 8] = [0; 8];
    uart_println!("Start DMACopy");
    krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
        ti: 0x10,
        src: &mut array0[0] as *mut _ as u32,
        dst: &mut array1[0] as *mut _ as u32,
        len: 32,
        stride: 0,
        ncba: 0,
        reserved1: 0,
        reserved2: 0,
    });
    for i in 0..8
    {
        uart_println!("array1[{:X}] = {:X}", i, array1[i]);
    }
}

fn test_entry() -> ! 
{
    test_dma();
    test_dma2();
    loop 
    {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(test_entry);