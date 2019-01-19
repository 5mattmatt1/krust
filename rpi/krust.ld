ENTRY(_start)
 
SECTIONS
{
    /* Starts at LOADER_ADDR. */
    . = 0x8000;
    __start = .;
    __text_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;
 
    __rodata_start = .;
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;
 
    __data_start = .;
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;
 
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096); /* align to page size */
    __bss_end = .;
 
    . = . + 0x800;
    stack_svc = .;
    . = . + 0x800;
    stack_und = .;
    . = . + 0x800;
    stack_abt = .;
    . = . + 0x800;
    stack_irq = .;
    . = . + 0x800;
    stack_fiq = .;
    . = . + 0x400000;
    stack_sys = .;

    __end = .;
}