switchGDT:
    lgdt [rdi]

    push 0x10 # Kernel data (new SS)
    push rsp
    pushf
    push 0x08 # Kernel code (new CS)

    lea rax, [rip + finishSwitchGDT]
    push rax

    iretq

finishSwitchGDT:
    pop rax

    mov ax, 0x10 # Kernel data
    mov ds, ax
    mov es, ax
    mov gs, ax
    mov fs, ax
    ret
