ENTRY(_start)

SECTIONS
{
    /* Tell the CPU exactly where QEMU drops it on boot */
    . = 0x40080000;

    /* Place the executable code first */
    .text : { 
        *(.text._start)
        *(.text*) 
    }

    /* Read-only data (like your text strings!) */
    .rodata : { *(.rodata*) }

    /* Read-write data */
    .data : { *(.data*) }

    /* Uninitialized memory */
    .bss : {
        *(.bss*)
        *(COMMON)
    }
}
