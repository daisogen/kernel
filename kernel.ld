ENTRY(kmain)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS
{
    . = 0xFFFF800000100000;

    __kernel_start = .;

    .stivale2hdr : ALIGN(4K) {
        KEEP(*(.stivale2hdr))
    }

    . = ALIGN(4K);
    .text : {
        __text_start = .;
        *(.text .text.*)
        . = ALIGN(4096);
        __text_end = .;
    }

    . = ALIGN(4K);
    .rodata : {
        __rodata_start = .;
        *(.rodata*)
        . = ALIGN(4096);
        __rodata_end = .;
    }

    . = ALIGN(4K);
    .data : {
        __data_start = .;
        *(.data .data.*)
        . = ALIGN(4096);
        __data_end = .;
    }

    . = ALIGN(4K);
    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        . = ALIGN(4096);
        __bss_end = .;
    }
}
