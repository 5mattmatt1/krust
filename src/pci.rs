const CONFIG_ADDRESS: u32 = 0xCF8;
const CONFIG_DATA: u32 = 0xCFC;

/* Only one header configuration */
/* Should use inheritance to allow for multiple configuration */
struct pci_device
{
    bus: u8,
    slot: u8,
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    prog_if: u8,
    subclass: u8,
    class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    bar0: u32,
    bar1: u32,
    bar2: u32,
    bar3: u32,
    bar4: u32,
    bar5: u32,
    carbus_cis_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: u32,
    capabilities_pointer: u16,
    reserved0: u16,
    reserved1: u32,
    interrupt_line: u8,
    interrupt_pin: u8,
    min_grant: u8,
    max_latency: u8
}

pub unsafe fn pci_config_read_word (bus: u32, slot: u32, func: u32, offset: u32) -> u16
{
    let address: u32;
    let mut tmp: u16;
 
    /* create configuration address as per Figure 1 */
    address = (bus << 16) | (slot << 11) |
              (func << 8) | offset | (0x80000000);
 
    /* write out the address */
    // x86_64::instructions::out(CONFIG_ADDRESS, address);
    // outl(0xCF8, address);
    asm!("outl %eax, %dx" :: "{dx}"(CONFIG_ADDRESS), "{eax}"(address) :: "volatile");

    /* read in the data */
    /* (offset & 2) * 8) = 0 will choose the first word of the 32 bits register */
    // tmp = (x86_64::instructions::inl(CONFIG_DATA) >> ((offset & 2) * 8)) & 0xffff;
    // let tmp: u32;
    asm!("inl %dx, %eax" : "={eax}"(tmp) : "{dx}"(CONFIG_DATA) :: "volatile");
    return tmp;
}

pub unsafe fn pci_config_read_double_word (bus: u32, slot: u32, func: u32, offset: u32) -> u32
{
    let address: u32;
    let mut tmp: u32;
 
    /* create configuration address as per Figure 1 */
    address = (bus << 16) | (slot << 11) |
              (func << 8) | offset | (0x80000000);
 
    /* write out the address */
    // x86_64::instructions::out(CONFIG_ADDRESS, address);
    // outl(0xCF8, address);
    asm!("outl %eax, %dx" :: "{dx}"(CONFIG_ADDRESS), "{eax}"(address) :: "volatile");

    /* read in the data */
    /* (offset & 2) * 8) = 0 will choose the first word of the 32 bits register */
    // tmp = (x86_64::instructions::inl(CONFIG_DATA) >> ((offset & 2) * 8)) & 0xffff;
    // let tmp: u32;
    asm!("inl %dx, %eax" : "={eax}"(tmp) : "{dx}"(CONFIG_DATA) :: "volatile");
    return tmp;
}

pub fn check_vendor(bus: u8, slot: u8) -> u16 
{
    let vendor: u16;
    let device: u16;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    vendor = unsafe { pci_config_read_word(bus as u32, slot as u32, 0, 0) };
    if vendor != 0xFFFF
    {
       device = unsafe {pci_config_read_word(bus as u32, slot as u32, 0, 2)};
    } 
    return vendor;
}

pub fn get_device(bus: u8, slot: u8) -> u16 
{
    let vendor: u16;
    let device: u16;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    if check_vendor(bus, slot) != 0xFFFF
    {
       device = unsafe {pci_config_read_word(bus as u32, slot as u32, 0, 2)};
       return device;
    }
    return 0xFFFF;
}

pub fn get_bar0(bus: u8, slot: u8) -> u32 
{
    let vendor: u16;
    let baseio: u32;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    if check_vendor(bus, slot) != 0xFFFF
    {
       baseio = unsafe {pci_config_read_double_word(bus as u32, slot as u32, 0x00, 0x10)};
       return baseio;
    }
    return 0xFFFF;
}

pub fn get_bar1(bus: u8, slot: u8) -> u32 
{
    let vendor: u16;
    let baseio: u32;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    if check_vendor(bus, slot) != 0xFFFF
    {
       baseio = unsafe {pci_config_read_double_word(bus as u32, slot as u32, 0x00, 0x14)};
       return baseio;
    }
    return 0xFFFF;
}

pub fn get_bar2(bus: u8, slot: u8) -> u32 
{
    let vendor: u16;
    let baseio: u32;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    if check_vendor(bus, slot) != 0xFFFF
    {
       baseio = unsafe {pci_config_read_double_word(bus as u32, slot as u32, 0x00, 0x18)};
       return baseio;
    }
    return 0xFFFF;
}

pub fn get_bar3(bus: u8, slot: u8) -> u32 
{
    let vendor: u16;
    let baseio: u32;
    let lbus = bus as u32;
    /* try and read the first configuration register. Since there are no */
    /* vendors that == 0xFFFF, it must be a non-existent device. */
    if check_vendor(bus, slot) != 0xFFFF
    {
       baseio = unsafe {pci_config_read_double_word(bus as u32, slot as u32, 0x00, 0x1C)};
       return baseio;
    }
    return 0xFFFF;
}