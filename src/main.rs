#[no-std]

const GPIO_ADDRESS: u32 = 0x20200000;

/* TODO: Later move to asm.rs */
// mov reg, #val puts the number val into the register named reg
// lsl reg, #val shifts the binary representation of the number in reg by val places to the left
// str reg, [dest, #val] stores the number in reg at the address given by dest+val
// lsl = logical shift left

/*
fn enable_output_asm() // This is the example for the sixteenth pin
{
    ldr r0, 0x20200000
    mov r1, #1 // on state
    lsl r1, #18
    str r1, [r0, #4]
    // should definitely pull str into asm.rs
}

fn turn_on_pin_asm() // This is the example for the sixteenth pin
{
    ldr r0, 0x20200000
    mov r1, #1 // on state
    lsl r1, #16
    str r1, [r0, #40]
}


fn turn_off_pin_asm() // This is the example for the sixteenth pin
{
    ldr r0, 0x20200000
    mov r1, #1 // on state
    lsl r1, #16
    str r1, [r0, #40]
}
*/

#[no-mangle]
fn _start()
{

}