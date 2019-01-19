/* There are roughly 386 (syscall_32.tbl) sys calls to implement into my pseudo linux to make it POSIX compatible */
// Linux kernel 2.x.y
// Used to be in arch/x86/kernel/syscall_table_32.S
// I wonder where it went off to
// Linux kernel 4.x.y
// arch/x86/entry/syscalls/syscall_64.tbl

// important: include/linux/syscalls.h


// 3.19.8
// #include <asm/syscalls_64.h>

/*
 * Kernel code should not call syscalls directly. (i.e., sys_xyzyyz())
 * instead it should use the use a function that works equivalently, such as ksys_xyzyyz().
 * These can be found in ksyscall.rs
 */