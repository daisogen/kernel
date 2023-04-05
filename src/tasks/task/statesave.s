# Like ../scheduler/kstatesave.s, but for tasks (if they're locked knowingly)
# This is not the state saver for preemption, there's another one

# rdi <- ptr to saved rip
# rsi <- ptr to saved rsp
# rdx <- ptr to SavedState (regs.rs)
save_state:
    mov rax, qword ptr [rsp]
    mov qword ptr [rdi], rax

    lea rax, [rsp + 8]
    mov qword ptr [rsi], rax

    pushfq
    pop rax
    mov qword ptr [rdx + 8*0], rax # rflags
    mov qword ptr [rdx + 8*1], 1 # rax=1 <==> returned!
    mov qword ptr [rdx + 8*2], rbx
    mov qword ptr [rdx + 8*3], rcx
    mov qword ptr [rdx + 8*4], rdx
    mov qword ptr [rdx + 8*5], rsi
    mov qword ptr [rdx + 8*6], rdi
    mov qword ptr [rdx + 8*7], rbp
    mov qword ptr [rdx + 8*8], r8
    mov qword ptr [rdx + 8*9], r9
    mov qword ptr [rdx + 8*10], r10
    mov qword ptr [rdx + 8*11], r11
    mov qword ptr [rdx + 8*12], r12
    mov qword ptr [rdx + 8*13], r13
    mov qword ptr [rdx + 8*14], r14
    mov qword ptr [rdx + 8*15], r15

    # Return now with rax=0 <==> saved
    xor eax, eax
    ret
