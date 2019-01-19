.section .text
.globl _start

_start:
    @ Get me out of fucking hypervisor right fucking now
    mrs r0,cpsr
    bic r0,r0,#0x1F
    orr r0,r0,#0x13
    msr spsr_cxsf,r0
    add r0,pc,#4
    msr ELR_hyp,r0
    eret

    mov r0, #0xD2    @ IRQ mode
    msr cpsr_c, r0   @ Put in IRQ mode, don't clear C bits
    mov sp, #0x8000  @ Set IRQ stack pointer
    mov r0, #0xD3    @ SVC mode
    msr cpsr_c, r0   @ Put in SVC mode, don't clear C bits
    mov sp, #0x7000  @ Set SVC stack pointer

    bl main

@ ------------------------------------------------------------------------------
@ Sets up stacks for all operating modes
@ ------------------------------------------------------------------------------
setup_stack:
    mov       r0, #0xD1       @ FIQ
    msr       cpsr, r0
    ldr       sp, =stack_fiq
    mov       r0, #0xD2       @ IRQ
    msr       cpsr, r0
    ldr       sp, =stack_irq
    mov       r0, #0xD7       @ ABT
    msr       cpsr, r0
    ldr       sp, =stack_abt
    mov       r0, #0xDB       @ UND
    msr       cpsr, r0
    ldr       sp, =stack_und
    mov       r0, #0xDF       @ SYS
    msr       cpsr, r0
    ldr       sp, =stack_sys
    mov       r0, #0xD3       @ SVC
    msr       cpsr, r0
    ldr       sp, =stack_svc
    mov pc, lr

@ ------------------------------------------------------------------------------
@ Enables the L1 cache
@ ------------------------------------------------------------------------------
setup_cache:
    mov       r0, #0
    mcr       p15, 0, r0, c7, c7, 0     @ Invalidate caches
    mcr       p15, 0, r0, c8, c7, 0     @ Invalidate TLB
    mrc       p15, 0, r0, c1, c0, 0
    ldr       r1, =0x1004
    orr       r0, r0, r1                @ Set L1 enable bit
    mcr       p15, 0, r0, c1, c0, 0
    mov pc, lr

@ ------------------------------------------------------------------------------
@ Enables the vectored floating point unit
@ ------------------------------------------------------------------------------
@ fmxr is not supported
@ setup_vfp:
@     mrc       p15, #0, r0, c1, c0, #2
@     orr       r0, r0, #0xF00000         @ Single + double precision
@     mcr       p15, #0, r0, c1, c0, #2
@     mov       r0, #0x40000000           @ Set VFP enable bit
@     fmxr      fpexc, r0
@     mov pc, lr

hang:
    b hang
