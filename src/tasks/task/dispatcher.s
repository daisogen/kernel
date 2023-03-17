saved_regs: .quad 0, 0, 0, 0, 0, 0
saved_rsp: .quad 0

try_restore_kernel_state:
    mov rax, qword ptr [rip + saved_rsp]
    test rax, rax
    jnz restore_kernel_state
    ret
restore_kernel_state:
    mov rsp, qword ptr [rip + saved_rsp]
    lea rax, [rip + saved_regs]
    mov rbp, qword ptr [rax + 8*0]
    mov rbx, qword ptr [rax + 8*1]
    mov r12, qword ptr [rax + 8*2]
    mov r13, qword ptr [rax + 8*3]
    mov r14, qword ptr [rax + 8*4]
    mov r15, qword ptr [rax + 8*5]

    # State has been consumed
    mov qword ptr [rip + saved_rsp], 0
    # Now we go back
    ret

dispatch_saving:
    mov qword ptr [rip + saved_rsp], rsp
    lea rax, [rip + saved_regs]
    mov qword ptr [rax + 8*0], rbp
    mov qword ptr [rax + 8*1], rbx
    mov qword ptr [rax + 8*2], r12
    mov qword ptr [rax + 8*3], r13
    mov qword ptr [rax + 8*4], r14
    mov qword ptr [rax + 8*5], r15
    # fallthrough to dispatch

# rdi <- SavedState*
# rsi <- rip
# rdx <- rsp
dispatch:
    # Stack frame for iretq
    push 0x10 # ss (KDATA)
    push rdx  # rsp
    mov rax, qword ptr [rdi]
    push rax  # rflags
    push 0x08 # cs (KCODE)
    push rsi  # rip

    # Restore registers
    mov r15, rdi
    mov rax, qword ptr [r15 + 8*1]
    mov rbx, qword ptr [r15 + 8*2]
    mov rcx, qword ptr [r15 + 8*3]
    mov rdx, qword ptr [r15 + 8*4]
    mov rsi, qword ptr [r15 + 8*5]
    mov rdi, qword ptr [r15 + 8*6]
    mov rbp, qword ptr [r15 + 8*7]
    mov r8, qword ptr  [r15 + 8*8]
    mov r9, qword ptr  [r15 + 8*9]
    mov r10, qword ptr [r15 + 8*10]
    mov r11, qword ptr [r15 + 8*11]
    mov r12, qword ptr [r15 + 8*12]
    mov r13, qword ptr [r15 + 8*13]
    mov r14, qword ptr [r15 + 8*14]
    mov r15, qword ptr [r15 + 8*15]

    # Here we go
    iretq
