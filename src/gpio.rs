pub fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!("nop"); }
    }
}

/* https://github.com/BrianSidebotham/arm-tutorial-rpi/blob/master/part-1/armc-02/armc-02.c */

// The GPIO registers base address.
const GPIO_BASE: u32 = 0x3F200000; // for raspi2 & 3, 0x20200000 for raspi1

// Controls actuation of pull up/down to ALL GPIO pins.
// const GPPUD: u32 = (GPIO_BASE + 0x94);

/* RPI 2 specific */
const LED_GPFSEL: isize = 4;
const LED_GPFBIT: u32 = 21;
const LED_GPSET: isize = 8;
const LED_GPCLR: isize = 11;
const LED_GPIO_BIT: u32 = 15;

// Is reversed on rpi3
pub fn turn_off_led()
{
    let gpio = GPIO_BASE as *const u32;
    let gpio_off = unsafe {gpio.offset(LED_GPCLR) as *mut u32};
    unsafe {*gpio_off = 1 << LED_GPIO_BIT};
}

pub fn turn_on_led()
{
    let gpio = GPIO_BASE as *const u32;
    let gpio_on = unsafe {gpio.offset(LED_GPSET) as *mut u32};
    unsafe {*gpio_on = 1 << LED_GPIO_BIT};    
}

pub fn enable_led()
{
    let gpio = GPIO_BASE as *const u32;
    let enable_led = unsafe {gpio.offset(LED_GPFSEL) as *mut u32};
    unsafe {*enable_led |= 1 << LED_GPFBIT};
}

// Use this sparingly
#[allow(dead_code)]
fn test_led()
{
    let gpio = GPIO_BASE as *const u32;
    let enable_led = unsafe {gpio.offset(LED_GPFSEL) as *mut u32};
    let gpio_on = unsafe {gpio.offset(LED_GPCLR) as *mut u32};
    let gpio_off = unsafe {gpio.offset(LED_GPSET) as *mut u32};
    unsafe {*enable_led |= 1 << LED_GPFBIT};

    loop 
    {
        sleep(500000);
        unsafe {*gpio_on = 1 << LED_GPIO_BIT};
        
        sleep(500000);
        unsafe {*gpio_off = 1 << LED_GPIO_BIT};
        
    }

}