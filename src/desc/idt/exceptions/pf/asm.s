asm_pf:
    # Time to push everything in case we get back
    # We're on IST1 so no issues with the stack
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax

    mov rdi, qword ptr [rsp + 15 * 8]
    mov rsi, rsp # Stack will be useful
    call pf_isr

    # Returned! Restore everything
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15

    # And discard the error code
    add rsp, 8

    iretq
