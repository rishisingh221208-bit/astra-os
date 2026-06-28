ENTRY(_start)

SECTIONS
{
    /* QEMU's ARM64 virtual machine loads the kernel right here */
    . = 0x40000000;

    .text : {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
    }

    /* Set up a safe 16KB stack space for our Rust functions to use */
    . = ALIGN(16);
    . += 0x4000; 
    __stack_top = .;
}

