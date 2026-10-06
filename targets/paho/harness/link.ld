ENTRY(reset_handler)

SECTIONS
{
    . = 0x08000000;

    .vectors : {
        KEEP(*(.vectors))
    }

    .text : {
        *(.text*)
        *(.rodata*)
    }

    _etext = .;

    .data 0x20000000 : AT(_etext) {
        *(.data*)
    }

    .bss ADDR(.data) + SIZEOF(.data) (NOLOAD) : {
        . = ALIGN(4);
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
    }

    /DISCARD/ : {
        *(.comment)
        *(.ARM.attributes)
    }
}
