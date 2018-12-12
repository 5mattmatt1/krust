#![no_std]
#![feature(core_intrinsics, lang_items, asm)]
#![no_main]

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

// The GPIO registers base address.
const GPIO_BASE: u32 = 0x3F200000; // for raspi2 & 3, 0x20200000 for raspi1

// Controls actuation of pull up/down to ALL GPIO pins.
const GPPUD: u32 = (GPIO_BASE + 0x94);

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

/*
fn uart_init()
{
    // Disablo UART0
    mmio_write(UART_CR, 0x00000000);

    // Setup the GPIO pin 14 && 15

    // Disable pull up/down for all GPIO pins & delay for 150 cycles
    mmio_write(GPPUD, 0x00000000);
}
*/

fn test_uart()
{
    write("Hello Rust Kernel world!");
    loop {
        writec(getc())
    }
}

fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!("nop"); }
    }
}

/* https://github.com/BrianSidebotham/arm-tutorial-rpi/blob/master/part-1/armc-02/armc-02.c */

/* RPI 2 specific */
const LED_GPFSEL: isize = 4;
const LED_GPFBIT: u32 = 21;
const LED_GPSET: isize = 8;
const LED_GPCLR: isize = 11;
const LED_GPIO_BIT: u32 = 15;

fn test_led()
{
    let gpio = GPIO_BASE as *const u32;
    let enable_led = unsafe {gpio.offset(LED_GPFSEL) as *mut u32};
    let gpio_on = unsafe {gpio.offset(LED_GPCLR) as *mut u32};
    let gpio_off = unsafe {gpio.offset(LED_GPSET) as *mut u32};
    unsafe {*enable_led |= (1 << LED_GPFBIT)};

    loop 
    {
        sleep(500000);
        unsafe {*gpio_on = 1 << LED_GPIO_BIT};
        
        sleep(500000);
        unsafe {*gpio_off = 1 << LED_GPIO_BIT};
        
    }

}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> !
{
    loop
    {

    }
}

#[no_mangle]
pub extern "C" fn _start() {
    // test_uart();
    test_led();
}