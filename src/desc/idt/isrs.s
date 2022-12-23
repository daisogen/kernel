.altmacro

.macro genisr x
    _isr\x:
        mov rdi, \x
        call default_isr
        hlt
.endmacro

.set i, 0
.rept 256
    genisr %i
    .set i, i + 1
.endr



ISRS:
    .macro putisr x
        .quad _isr\x
    .endmacro

    .set i, 0
    .rept 256
        putisr %i
        .set i, i + 1
    .endr
