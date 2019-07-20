/// https://stackoverflow.com/questions/31990487/how-to-cleanly-exit-qemu-after-executing-bare-metal-program-without-user-interve
/// Put this behind a cfg later...
// pub fn semihost_qemu()
// {
//     unsafe {
//     asm!("mov r0, $0" :: "r"(0x18) :: "volatile");    // angel_SWIreason_ReportException
//     asm!("mov r1, $0" :: "r"(0x20026) :: "volatile"); // ADP_Stopped_ApplicationExit
//     asm!("svc 0x00123456"); // // make semihosting call
//     }
// }

/// https://stackoverflow.com/questions/31990487/how-to-cleanly-exit-qemu-after-executing-bare-metal-program-without-user-interve/49930361#49930361
/// https://developer.arm.com/docs/100863/latest
pub fn semihost_qemu()
{
    unsafe
    {
        /* 0x20026 == ADP_Stopped_ApplicationExit */
        asm!("mov x1, #0x26");
        asm!("movk x1, #2, lsl #16");
        asm!("str x1, [sp,#0]");

        /* Exit status code. Host QEMU process exits with that status. */
        asm!("mov x0, #0");
        asm!("str x0, [sp,#8]");

        /* x1 contains the address of parameter block.
        * Any memory address could be used. */
        asm!("mov x1, sp");

        /* SYS_EXIT */
        asm!("mov w0, #0x18");

        /* Do the semihosting call on A64. */
        asm!("hlt 0xf000");
    }
}