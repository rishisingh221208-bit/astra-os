ENTRY(_start)

SECTIONS
{
    /* QEMU virt machine RAM starts here */
    . = 0x40000000;

    .text :
    {
        /* Ensure our Assembly entry point is placed first */
        *(.text._start)
        *(.text*)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .data :
    {
        *(.data*)
    }

    .bss :
    {
        __bss_start = .;
        *(.bss*)
        __bss_end = .;
    }

    /* Reserve a 16KB stack and define the __stack_top symbol */
    . = ALIGN(4096);
    . += 0x4000;
    __stack_top = .;
}
