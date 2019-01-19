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

// Maybe turn this into an enum...
pub const PUD_OFF: u32 = 0;
pub const PUD_DOWN: u32 = 1;
pub const PUD_UP: u32 = 2;

const FSEL_OFFSET: isize = 0;   // 0x0000
const PULLUPDN_OFFSET: isize = 37;  // 0x0094 / 4
const PULLUPDNCLK_OFFSET: isize = 38;  // 0x0098 / 4

pub const INPUT: u32 = 1; // is really 0 for control register!
pub const OUTPUT: u32 = 0; // is really 1 for control register!

/* RPI 2 specific */
const LED_GPFSEL: isize = 4;
const LED_GPFBIT: u32 = 21;
const LED_GPSET: isize = 8;
const LED_GPCLR: isize = 11;
const LED_GPIO_BIT: u32 = 15;

const SET_OFFSET: u32 = 7;
const CLR_OFFSET: u32 = 10;
// #define SET_OFFSET                  7   // 0x001c / 4
// #define CLR_OFFSET                  10  // 0x0028 / 4

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

/*
void set_pullupdn(int gpio, int pud)
{
    int clk_offset = PULLUPDNCLK_OFFSET + (gpio >> 5);
    int shift = gpio & 31;
    
    int32_t * pullupdn = (int32_t *) (gpio_map + PULLUPDN_OFFSET);
    int32_t * gpio_clk = (int32_t *) (gpio_map + clk_offset);
    *pullupdn &= ~3;
    if (pud != PUD_OFF)
    {
        *pullupdn &= pud;
    }

    short_wait();
    *gpio_clk = 1 << shift;
    short_wait();
    *pullupdn &= ~3;
    *gpio_clk = 0;
}
*/

/*
void output_gpio(int gpio, int value)
{
    int offset, shift;

    if (value) // value == HIGH
        offset = SET_OFFSET + (gpio/32);
    else       // value == LOW
       offset = CLR_OFFSET + (gpio/32);

    shift = (gpio%32);

    *(gpio_map+offset) = 1 << shift;
}

void set_pullupdn(int gpio, int pud)
{
    int clk_offset = PULLUPDNCLK_OFFSET + (gpio/32);
    int shift = (gpio%32);

    if (pud == PUD_DOWN)
        *(gpio_map+PULLUPDN_OFFSET) = (*(gpio_map+PULLUPDN_OFFSET) & ~3) | PUD_DOWN;
    else if (pud == PUD_UP)
        *(gpio_map+PULLUPDN_OFFSET) = (*(gpio_map+PULLUPDN_OFFSET) & ~3) | PUD_UP;
    else  // pud == PUD_OFF
        *(gpio_map+PULLUPDN_OFFSET) &= ~3;

    short_wait();
    *(gpio_map+clk_offset) = 1 << shift;
    short_wait();
    *(gpio_map+PULLUPDN_OFFSET) &= ~3;
    *(gpio_map+clk_offset) = 0;
}

void setup_gpio(int gpio, int direction, int pud)
{
    int offset = FSEL_OFFSET + (gpio/10);
    int shift = (gpio%10)*3;

    set_pullupdn(gpio, pud);
    if (direction == OUTPUT)
        *(gpio_map+offset) = (*(gpio_map+offset) & ~(7<<shift)) | (1<<shift);
    else  // direction == INPUT
        *(gpio_map+offset) = (*(gpio_map+offset) & ~(7<<shift));
}

void setup_gpio(int gpio, int direction, int pud)
{
    int offset = FSEL_OFFSET + (gpio/10);
    int shift = (gpio % 10)*3;

    set_pullupdn(gpio, pud);
    if (direction == OUTPUT)
        *(gpio_map+offset) = (*(gpio_map+offset) & ~(7<<shift)) | (1<<shift);
    else  // direction == INPUT
        *(gpio_map+offset) = (*(gpio_map+offset) & ~(7<<shift));
}
*/

fn set_pullupdn(gpio: u32, pud: u32)
{
    let gpio_addr = GPIO_BASE as *const u32;
    let clk_offset: isize = PULLUPDNCLK_OFFSET + (gpio as isize >> 5); // gpio / 32
    let shift: u32 = gpio & 31; // gpio % 32

    let tmp: u32 = 0;
    
    // Could benefit from vol::read32 and vol::write32
    let pullupdn = unsafe {gpio_addr.offset(PULLUPDN_OFFSET) as *mut u32};
    let gpio_clk = unsafe {gpio_addr.offset(clk_offset) as * mut u32};

    unsafe { *pullupdn &= !3; }

    /*
    Could possibly be more efficient with a:
    if pud != PUD_OFF
    {
        unsafe { *pullupdn |= pud; }
    }
    */   
    if pud == PUD_DOWN
    {
        unsafe { *pullupdn |= PUD_DOWN; }
    } else if pud == PUD_UP
    {
        unsafe { *pullupdn |= PUD_UP; }
    }

    // short_wait();
    unsafe {*gpio_clk = 1 << shift; }
    // short_wait();
    unsafe {*pullupdn &= !3; }
    unsafe {*gpio_clk = 0; }
}

pub fn setup_gpio(gpio: u32, d10_gpio: u32, dir: u32, pud: u32)
{
    let gpio_addr = GPIO_BASE as *const u32;
    // Need to set up aebi_div
    let offset: isize = FSEL_OFFSET + d10_gpio as isize; // (gpio as isize / 10);
    let shift: u32 = (gpio & 9) * 3;

    set_pullupdn(gpio, pud);
    let pin =  unsafe { gpio_addr.offset(offset as isize) as *mut u32 };
    
    if dir == OUTPUT
    {
        unsafe { *pin &= !(7 << shift) | (1 << shift); }
    } else 
    {
        unsafe { *pin &= !(7 << shift); }
    }
    
}

pub fn output_gpio(gpio: u32, value: bool)
{
    let gpio_addr = GPIO_BASE as *const u32;
    let offset: u32;
    let shift: u32;

    if value // VALUE == HIGH
    {
        offset = SET_OFFSET + (gpio >> 5);
    } else // VALUE == LOW
    {
        offset = CLR_OFFSET + (gpio >> 5);
    }

    shift = gpio & 31; // gpio % 32


    let pin = unsafe {gpio_addr.offset(offset as isize) as *mut u32};
    // Might be a better way to do this
    unsafe { *pin = 1 << shift; }
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