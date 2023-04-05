saved_regs: .quad 0, 0, 0, 0, 0, 0
saved_rsp: .quad 0
saved_jmp: .quad 0

has_saved_kernel_state:
    mov rax, qword ptr [rip + saved_rsp]
    test rax, rax
    jnz .some
    ret # returns 0
    .some:
    xor eax, eax
    inc eax
    ret #returns 1

restore_kernel_state:
    mov rsp, qword ptr [rip + saved_rsp]

    lea rax, [rip + saved_regs]
    mov rbp, qword ptr [rax + 8*0]
    mov rbx, qword ptr [rax + 8*1]
    mov r12, qword ptr [rax + 8*2]
    mov r13, qword ptr [rax + 8*3]
    mov r14, qword ptr [rax + 8*4]
    mov r15, qword ptr [rax + 8*5]

    mov rdi, [rip + saved_jmp]

    # State has been consumed
    mov qword ptr [rip + saved_rsp], 0

    # Now we go back
    xor eax, eax
    inc eax # rax=1 <==> kernel state restored
    jmp rdi

# This is only safe in singlethread!
# rdi <- Return address
save_kernel_state:
    mov rax, qword ptr [rsp]
    mov qword ptr [rip + saved_jmp], rax

    lea rax, [rsp + 8]
    mov qword ptr [rip + saved_rsp], rax

    lea rax, [rip + saved_regs]
    mov qword ptr [rax + 8*0], rbp
    mov qword ptr [rax + 8*1], rbx
    mov qword ptr [rax + 8*2], r12
    mov qword ptr [rax + 8*3], r13
    mov qword ptr [rax + 8*4], r14
    mov qword ptr [rax + 8*5], r15

    xor eax, eax # rax=0 <==> kernel state saved OK
    ret
