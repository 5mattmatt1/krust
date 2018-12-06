use crate::{gdt, print, println, hlt_loop};
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
// use crate::println;
use lazy_static::lazy_static;

use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET; // new
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x1; // new
/* +2 is slave pci */
pub const SLAVE_PCI_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x2;
pub const THIRD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x3;
pub const FORTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x4;
pub const FIFTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x5;
pub const SIXTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x6;
pub const SEPTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x7;
pub const OCTTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x8;
pub const NINTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0x9;
pub const TENTH_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0xA;
pub const NIC_INTERRUPT_ID: u8 = PIC_1_OFFSET + 0xB;


pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); 
        }
        idt[usize::from(TIMER_INTERRUPT_ID)]
            .set_handler_fn(timer_interrupt_handler); // new
        idt[usize::from(KEYBOARD_INTERRUPT_ID)]
            .set_handler_fn(keyboard_interrupt_handler);
        idt[usize::from(SLAVE_PCI_INTERRUPT_ID)]
            .set_handler_fn(slave_pci_interrupt_handler);
        idt[usize::from(THIRD_INTERRUPT_ID)]
            .set_handler_fn(third_interrupt_handler);
        idt[usize::from(FORTH_INTERRUPT_ID)]
            .set_handler_fn(forth_interrupt_handler);
        idt[usize::from(FIFTH_INTERRUPT_ID)]
            .set_handler_fn(fifth_interrupt_handler);
        idt[usize::from(SIXTH_INTERRUPT_ID)]
            .set_handler_fn(sixth_interrupt_handler);
        idt[usize::from(SEPTH_INTERRUPT_ID)]
            .set_handler_fn(septh_interrupt_handler);
        idt[usize::from(OCTTH_INTERRUPT_ID)]
            .set_handler_fn(octth_interrupt_handler);
        idt[usize::from(NINTH_INTERRUPT_ID)]
            .set_handler_fn(ninth_interrupt_handler);
        idt[usize::from(TENTH_INTERRUPT_ID)]
            .set_handler_fn(tenth_interrupt_handler);
        idt[usize::from(NIC_INTERRUPT_ID)]
            .set_handler_fn(nic_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use x86_64::instructions::port::Port;
    use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut keyboard = KEYBOARD.lock();
    let port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}

extern "x86-interrupt" fn slave_pci_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Slave pci interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(SLAVE_PCI_INTERRUPT_ID) }
}

extern "x86-interrupt" fn third_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Third interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(THIRD_INTERRUPT_ID) }
}

extern "x86-interrupt" fn forth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Forth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(FORTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn fifth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Fifth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(FIFTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn sixth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Sixth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(SIXTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn septh_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Septh interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(SEPTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn octth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Octth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(OCTTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn ninth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Ninth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(NINTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn tenth_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Tenth interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(TENTH_INTERRUPT_ID) }
}

extern "x86-interrupt" fn nic_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use crate::serial_println;
    serial_println!("Network interrupt!");
    unsafe { PICS.lock().notify_end_of_interrupt(NIC_INTERRUPT_ID) }
}