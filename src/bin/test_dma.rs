#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate krust;

fn test_src_inc_dst_inc()
{
    uart_println!("test_src_inc_dst_inc");
    let mut array0: [u32; 8] = [0xAC, 0xDC, 0xFF, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
    let mut array1: [u32; 8] = [0; 8];
    krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
        ti: 0x331,
        src: &mut array0[0] as *mut _ as u32,
        dst: &mut array1[0] as *mut _ as u32,
        len: 32,
        stride: 0,
        ncba: 0,
        reserved1: 0,
        reserved2: 0,
    });
    uart_println!("{:X?}", array1);
}

fn test_src_dst_inc()
{
    uart_println!("test_src_dst_inc");
    let mut array0: [u32; 8] = [0xAC, 0xDC, 0xFF, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
    let mut array1: [u32; 8] = [0; 8];
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
    uart_println!("{:X?}", array1);
}

// Not a real test as I'm not 100% sure how to actually interact with
// 2D mode.
// fn test_2d_mode()
// {
//     uart_println!("test_2d_mode");
//     let mut array0: [u32; 8] = [0xAC, 0xDC, 0xFF, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
//     let mut array1: [u32; 8] = [0; 8];
//     krust::dma::DMACHANNEL0.copy(&mut krust::dma::DmaControlBlock {
//         ti: 0x02,
//         src: &mut array0[7] as *mut _ as u32,
//         dst: &mut array1[7] as *mut _ as u32,
//         len: 16 << 16 | 4,
//         stride: 4  << 16 | 4,
//         ncba: 0,
//         reserved1: 0,
//         reserved2: 0,
//     });
//     uart_println!("{:X?}", array1);
// }

fn test_entry() -> ! 
{
    test_src_inc_dst_inc();
    test_src_dst_inc();
    krust::qemu::semihost_qemu();
    loop 
    {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(test_entry);