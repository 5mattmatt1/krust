use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    uart_println!("Kernel Panic! {}", info);
    loop {}
}