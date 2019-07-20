#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate krust;

fn test_sdhci_init()
{
    use krust::sdhci::{SDIO, RPISDIO};
    let mut sdio = RPISDIO::new();

    sdio.init();
    uart_println!("SDIO Init!");
}

fn test_entry() -> ! 
{
    test_sdhci_init();
    loop 
    {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(test_entry);