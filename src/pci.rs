use heapless::String;
use heapless::consts::*;
use crate::{serial_println, println, asm};

const CONFIG_ADDRESS: u32 = 0xCF8;
const CONFIG_DATA: u32 = 0xCFC;

/* References */
/* arch/x86/pci/direct.c */
/* https://wiki.osdev.org/PCI */

/* Header Type := Any */
/* Offset := 0x00 */
const VENDOR_ID_MASK: u32 = 0x0000FFFF;
const VENDOR_ID_SHIFT: u32 = 0x0;
const DEVICE_ID_MASK: u32 = 0xFFFF0000;
const DEVICE_ID_SHIFT: u32 = 0x10;
/* Offset := 0x04 */
const COMMAND_MASK: u32 = 0x0000FFFF;
const COMMAND_SHIFT: u32 = 0x0;
const STATUS_MASK: u32 = 0xFFFF0000;
const STATUS_SHIFT: u32 = 0x10;
/* Offset := 0x08 */
const REVISION_ID_MASK: u32 = 0x000000FF;
const REVISION_ID_SHIFT: u32 = 0x0;
const PROG_IF_MASK: u32 = 0x0000FF00;
const PROG_IF_SHIFT: u32 = 0x8;
const SUBCLASS_MASK: u32 = 0x00FF0000;
const SUBCLASS_SHIFT: u32 = 0x10;
const CLASS_MASK: u32 = 0xFF000000;
const CLASS_SHIFT: u32 = 0x18;
/* Offset := 0x0C */
const CACHE_LINE_SIZE_MASK: u32 = 0x000000FF;
const CACHE_LINE_SIZE_SHIFT: u32 = 0x0;
const LATENCY_TIMER_MASK: u32 = 0x0000FF00;
const LATENCY_TIMER_SHIFT: u32 = 0x8;
const HEADER_TYPE_MASK: u32 = 0x00FF0000;
const HEADER_TYPE_SHIFT: u32 = 0x10;
const BIST_MASK: u32 = 0xFF000000;
const BIST_SHIFT: u32 = 0x18;
/* Header Type := 00h */
/* Offset := 0x2C */
const SUBSYSTEM_VENDOR_ID_MASK: u32 = 0x0000FFFF;
const SUBSYSTEM_VENDOR_ID_SHIFT: u32 = 0x0;
const SUBSYSTEM_ID_MASK: u32 = 0xFFFF0000;
const SUBSYSTEM_ID_SHIFT: u32 = 0x10;
/* 8-31 bits on offset 0x34 are reserved */
/* Offset := 0x34 */
const CAPABILITIES_POINTER_MASK: u32 = 0x000000FF;
const CAPABILITIES_POINTER_SHIFT: u32 = 0x0;
/* Offset 0x38 is entirely reserved */
/* Offset := 0x3C */
const INTERRUPT_LINE_MASK: u32 = 0x000000FF;
const INTERRUPT_LINE_SHIFT: u32 = 0x0;
const INTERRUPT_PIN_MASK: u32 = 0x0000FF00;
const INTERRUPT_PIN_SHIFT: u32 = 0x8;
const MIN_GRANT_MASK: u32 = 0x00FF0000;
const MIN_GRANT_SHIFT: u32 = 0x10;
const MAX_LATENCY_MASK: u32 = 0xFF000000;
const MAX_LATENCY_SHIFT: u32 = 0x18;
/* Memory block offsets */
// const BLOCK0_OFFSET: u32 = 0x00;
// const BLOCK1_OFFSET: u32 = 0x04;
// const BLOCK2_OFFSET: u32 = 0x08;
// const BLOCK3_OFFSET: u32 = 0x0C;
/* Only one header configuration */
/* Should use inheritance to allow for multiple configuration */
struct PciDevice
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
    /* Below is for 00h */
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


// & 0xFC
// Masks the register so that it is in the range {0}u[0x4, 0xFF]
// This is due to the offsets in the PCI table being done in sets of 0x4 (a.k.a 32 bits)

// & 0xF00
// Masks the register so that it is in the range {0}u[0xFF, 0xFFFF]

pub fn pci_conf1_address(bus: u8, devfn: u8, reg: u16) -> u32
{
    let lreg: u32 = bus as u32;
    let ldevfn: u32 = devfn as u32;
    let lreg: u32 = devfn as u32;
    let lbus: u32 = bus as u32;
    return 0x80000000 | ((lreg & 0xF00) << 16) | (lbus << 16) | (ldevfn << 8) | (lreg & 0xFC);
}

pub fn pci_slconf1_address(bus: u8, slot: u8, devfn: u8, reg: u16) -> u32
{
    let lreg: u32 = reg as u32;
    let ldevfn: u32 = devfn as u32;
    let lbus: u32 = bus as u32;
    let lslot: u32 = slot as u32;
    return 0x80000000 | ((lreg & 0xF00) << 16) | (lbus << 16) | (lslot << 11) | (ldevfn << 8) | (lreg & 0xFC);
}

pub unsafe fn pci_conf1_read(bus: u8, devfn: u8, reg: u16) -> u32
{
    // Should look into a u12
    let address: u32;
    // let tmp: u32;
    address = pci_conf1_address(bus, devfn, reg);
    serial_println!("PCI_CONF1_ADDRESS: {:X}", address);
    // asm!("outl %eax, %dx" :: "{dx}"(CONFIG_ADDRESS), "{eax}"(address) :: "volatile");
    // asm!("inl %dx, %eax" : "={eax}"(tmp) : "{dx}"(CONFIG_DATA) :: "volatile");
    asm::outl(CONFIG_ADDRESS, address);
    return asm::inl(CONFIG_DATA);
}

pub unsafe fn pci_slconf1_read(bus: u8, slot: u8, devfn: u8, reg: u16) -> u32
{
    let address: u32;
    let tmp: u32;
    address = pci_slconf1_address(bus, slot, devfn, reg);
    // asm::outl(CONFIG_ADDRESS, address);
    // return asm::inl(CONFIG_DATA);
    /* 
     * Difference between these two... 
     * Maybe some register clobbering is happening
     */
    asm!("outl %eax, %dx" :: "{dx}"(CONFIG_ADDRESS), "{eax}"(address) :: "volatile");
    asm!("inl %dx, %eax" : "={eax}"(tmp) : "{dx}"(CONFIG_DATA) :: "volatile");
    return tmp;
}

pub unsafe fn pci_info_dump(bus: u8, slot: u8)
{
    /* Reads memory blocks for PCI bus */
    let block0: u32 = pci_slconf1_read(bus, slot, 0, 0x00);
    let block1: u32 = pci_slconf1_read(bus, slot, 0, 0x04);
    let block2: u32 = pci_slconf1_read(bus, slot, 0, 0x08);
    let block3: u32 = pci_slconf1_read(bus, slot, 0, 0x0C);
    
    let vendor_id: u32 = (block0 & VENDOR_ID_MASK) >> VENDOR_ID_SHIFT;
    let device_id: u32 = (block0 & DEVICE_ID_MASK) >> DEVICE_ID_SHIFT;

    let command: u32 = (block1 & COMMAND_MASK) >> COMMAND_SHIFT;
    let status: u32 = (block1 & STATUS_MASK) >> STATUS_SHIFT;

    let revision_id: u32 = (block2 & REVISION_ID_MASK) >> REVISION_ID_SHIFT;
    let prog_if: u32 = (block2 & PROG_IF_MASK) >> PROG_IF_SHIFT;
    let subclass_code: u32 = (block2 & SUBCLASS_MASK) >> SUBCLASS_SHIFT;
    let class_code: u32 = (block2 & CLASS_MASK) >> CLASS_SHIFT;

    let cache_line_size: u32 = (block3 & CACHE_LINE_SIZE_MASK) >> CACHE_LINE_SIZE_SHIFT;
    let latency_timer: u32 = (block3 & LATENCY_TIMER_MASK) >> LATENCY_TIMER_SHIFT;
    let header_type: u32 = (block3 & HEADER_TYPE_MASK) >> HEADER_TYPE_SHIFT;
    let bist: u32 = (block3 & BIST_MASK) >> BIST_SHIFT;

    println!("Vendor id: 0x{:X}", vendor_id);
    // println!((vendor_id != 0xFFFFFFFF) ? "Valid vendor id" : "Invalid vendor id");
    println!("Device id: 0x{:X}", device_id);

    println!("Command: 0x{:X}", command);
    println!("Status: 0x{:X}", status);

    println!("Revision Id: 0x{:X}", revision_id);
    println!("Prog IF: 0x{:X}", prog_if);
    println!("Class code: 0x{:X}", class_code);
    println!("Subclass code: 0x{:X}", subclass_code);

    println!("Cache line size: 0x{:X}", cache_line_size);
    println!("Latency timer: 0x{:X}", latency_timer);
    println!("Header type: 0x{:X}", header_type);
    println!("BIST: 0x{:X}", bist);
    let driver_info: (String<U24>, String<U24>, String<U24>);
    driver_info = pci_parsedriver(class_code as u8, 
                                  subclass_code as u8,
                                  prog_if as u8);
    println!("Class: {}", driver_info.0.as_str());
    println!("Subclass: {}", driver_info.1.as_str());
    println!("Prog IF: {}", driver_info.2.as_str());
    match header_type
    {
        0x00 => pci_info_dump00h(bus, slot),
        0x01 => pci_info_dump01h(bus, slot),
        0x02 => pci_info_dump02h(bus, slot),
        _ => println!("Unknown header type"),
    }
}

pub unsafe fn pci_info_dump00h(bus: u8, slot: u8)
{
    // Note: should probably change all these println!'s to 
    // serial_println!'s
    let block4: u32 = pci_slconf1_read(bus, slot, 0, 0x10);
    let block5: u32 = pci_slconf1_read(bus, slot, 0, 0x14);
    let block6: u32 = pci_slconf1_read(bus, slot, 0, 0x18);
    let block7: u32 = pci_slconf1_read(bus, slot, 0, 0x1C);
    let block8: u32 = pci_slconf1_read(bus, slot, 0, 0x20);
    let block9: u32 = pci_slconf1_read(bus, slot, 0, 0x24);
    let blockA: u32 = pci_slconf1_read(bus, slot, 0, 0x28);
    let blockB: u32 = pci_slconf1_read(bus, slot, 0, 0x2C);
    let blockC: u32 = pci_slconf1_read(bus, slot, 0, 0x30);
    let blockD: u32 = pci_slconf1_read(bus, slot, 0, 0x34);
    // let blockE: u32 = pci_slconf1_read(bus, slot, 0, 0x38);
    let blockF: u32 = pci_slconf1_read(bus, slot, 0, 0x3C);
    println!("Base address #0: 0x{:X}", block4);
    println!("Base address #1: 0x{:X}", block5);
    println!("Base address #2: 0x{:X}", block6);
    println!("Base address #3: 0x{:X}", block7);
    println!("Base address #4: 0x{:X}", block8);
    println!("Base address #5: 0x{:X}", block9);
    println!("Cardbus CIS Pointer 0x{:X}", blockA);
    let subsystem_id: u32 = (blockB & SUBSYSTEM_ID_MASK) >> SUBSYSTEM_ID_SHIFT;
    let subsystem_vendor_id: u32 = (blockB & SUBSYSTEM_VENDOR_ID_MASK) >> SUBSYSTEM_VENDOR_ID_SHIFT;
    println!("Subsystem id: 0x{:X}", subsystem_id);
    println!("Subsystem vendor id: 0x{:X}", subsystem_vendor_id);
    println!("Expansion ROM Address: 0x{:X}", blockC);
    let capabilities_pointer: u32 = (blockD & CAPABILITIES_POINTER_MASK) >> CAPABILITIES_POINTER_SHIFT;
    println!("Capabilities pointer: 0x{:X}", capabilities_pointer);
    let interrupt_line: u32 = (blockF & INTERRUPT_LINE_MASK) >> INTERRUPT_LINE_SHIFT;
    let interrupt_pin: u32 = (blockF & INTERRUPT_PIN_MASK) >> INTERRUPT_PIN_SHIFT;
    let min_grant: u32 = (blockF & MIN_GRANT_MASK) >> MIN_GRANT_SHIFT;
    let max_latency: u32 = (blockF & MAX_LATENCY_MASK) >> MAX_LATENCY_SHIFT;
    println!("Interrupt line: 0x{:X}", interrupt_line);
    println!("Interrupt pin: 0x{:X}", interrupt_pin);
    println!("Min Grant: 0x{:X}", min_grant);
    println!("Max Latency: 0x{:X}", max_latency);
}

pub unsafe fn pci_info_dump01h(bus: u8, slot: u8)
{
    let block4: u32 = pci_slconf1_read(bus, slot, 0, 0x10);
    let block5: u32 = pci_slconf1_read(bus, slot, 0, 0x14);
    let block6: u32 = pci_slconf1_read(bus, slot, 0, 0x18);
    let block7: u32 = pci_slconf1_read(bus, slot, 0, 0x1C);
    let block8: u32 = pci_slconf1_read(bus, slot, 0, 0x20);
    let block9: u32 = pci_slconf1_read(bus, slot, 0, 0x24);
    let blockA: u32 = pci_slconf1_read(bus, slot, 0, 0x28);
    let blockB: u32 = pci_slconf1_read(bus, slot, 0, 0x2C);
    let blockC: u32 = pci_slconf1_read(bus, slot, 0, 0x30);
    let blockD: u32 = pci_slconf1_read(bus, slot, 0, 0x34);
    let blockE: u32 = pci_slconf1_read(bus, slot, 0, 0x38);
    let blockF: u32 = pci_slconf1_read(bus, slot, 0, 0x3C);
}

pub unsafe fn pci_info_dump02h(bus: u8, slot: u8)
{
    let block04: u32 = pci_slconf1_read(bus, slot, 0, 0x10);
    let block05: u32 = pci_slconf1_read(bus, slot, 0, 0x14);
    let block06: u32 = pci_slconf1_read(bus, slot, 0, 0x18);
    let block07: u32 = pci_slconf1_read(bus, slot, 0, 0x1C);
    let block08: u32 = pci_slconf1_read(bus, slot, 0, 0x20);
    let block09: u32 = pci_slconf1_read(bus, slot, 0, 0x24);
    let block0A: u32 = pci_slconf1_read(bus, slot, 0, 0x28);
    let block0B: u32 = pci_slconf1_read(bus, slot, 0, 0x2C);
    let block0C: u32 = pci_slconf1_read(bus, slot, 0, 0x30);
    let block0D: u32 = pci_slconf1_read(bus, slot, 0, 0x34);
    let block0E: u32 = pci_slconf1_read(bus, slot, 0, 0x38);
    let block0F: u32 = pci_slconf1_read(bus, slot, 0, 0x3C);
    let block10: u32 = pci_slconf1_read(bus, slot, 0, 0x40);
    let block11: u32 = pci_slconf1_read(bus, slot, 0, 0x44);
}

pub fn pci_parsedriver(class: u8, subclass: u8, 
    prog_if: u8) -> (String<U24>, String<U24>, String<U24>)
{
    /* No actual support for prog_if yet */
    let mut class_str: String<U24> = String::from("");
    let mut subclass_str: String<U24> = String::from("");
    let mut prog_if_str: String<U24> = String::from("");
    match class
    {
        0x0 => 
        {
            class_str = String::from("Unclassified");
            match subclass
            {
                0x0 => subclass_str = String::from("Non-VGA-Compatible device"),
                0x1 => subclass_str = String::from("VGA-Compatible Device"),
                0xFF => subclass_str = String::from("Invalid device"),
                _ => subclass_str = String::from("Unknown subclass"),
            }
        },
        0x1 =>
        {
            class_str = String::from("Mass Storage Controller");
            match subclass
            {
                0x0 => subclass_str = String::from("SCSI Bus Controller"),
                0x1 => subclass_str = String::from("IDE Controller"),
                0x2 => subclass_str = String::from("Floppy Disk Controller"),
                0x3 => subclass_str = String::from("IPI Bus Controller"),
                0x4 => subclass_str = String::from("RAID Controller"),
                0x5 => subclass_str = String::from("ATA Controller"),
                0x6 => subclass_str = String::from("Serial ATA"),
                0x7 => subclass_str = String::from("Serial Attached SCSI"),
                0x8 => subclass_str = String::from("Non-Volatile Memory Controller"),
                0x80 => subclass_str = String::from("Other"),
                _ => subclass_str = String::from("Unknown Controller"),
            }
        },
        0x02 => 
        {
            class_str = String::from("Network Controller");
            match subclass
            {
                0x00 => subclass_str = String::from("Ethernet Controller"),
                0x01 => subclass_str = String::from("Token Ring Controller"),
                0x02 => subclass_str = String::from("FDDI Controller"),
                0x03 => subclass_str = String::from("ATM Controller"),
                0x04 => subclass_str = String::from("ISDN Controller"),
                0x05 => subclass_str = String::from("WorldFip Controller"),
                0x06 => subclass_str = String::from("PICMG 2.14 Multi Computing"),
                0x07 => subclass_str = String::from("Infiniband Controller"),
                0x08 => subclass_str = String::from("Fabric Controller"),
                0x80 => subclass_str = String::from("Other"),
                _ => subclass_str = String::from("Unknown device"),
            }
        },
        0x03 =>
        {
            class_str = String::from("Display Controller");
        },
        0x04 =>
        {
            class_str = String::from("Multimedia Controller");
        },
        0x05 =>
        {
            class_str = String::from("Memory Controller");
        },
        0x06 =>
        {
            class_str = String::from("Bridge Device");
        },
        0x07 =>
        {
            class_str = String::from("Simple communication Controller");
        },
        0x08 =>
        {
            class_str = String::from("Base system peripheral");
        },
        0x09 =>
        {
            class_str = String::from("Input Device Controller");
        },
        0x0A =>
        {
            class_str = String::from("Docking Station");
        },
        0x0B =>
        {
            class_str = String::from("Processor");
        },
        0x0C =>
        {
            class_str = String::from("Serial Bus Controller");
        },
        0x0D =>
        {
            class_str = String::from("Wireless Controller");
        },
        0x0E =>
        {
            class_str = String::from("Intelligent Controller");
        },
        0x0F =>
        {
            class_str = String::from("Satellite Communication Controller");
        },
        0x10 =>
        {
            class_str = String::from("Encryption Controller");
        },
        0x11 =>
        {
            class_str = String::from("Signal Processing Controller");
        },
        0x12 =>
        {
            class_str = String::from("Processing Accelerator");
        },
        0x13 =>
        {
            class_str = String::from("Non-Essential Instrumentation");
        },
        0x40 =>
        {
            class_str = String::from("Co-Processor");
        },
        0xFF =>
        {
            class_str = String::from("Unassigned Class");
        },
        _ =>
        {
            class_str = String::from("Unknown Controller");
            subclass_str = String::from("Unknown Controller");
            prog_if_str = String::from("Unknown Controller");
        }
    }
    return (class_str, subclass_str, prog_if_str);
}