BITS 64

    global  _start

    section .text

%macro  align_nops 1+ 

  %%i: %1
  %%endi: 
        times 10-(%%endi-%%i) nop 

%endmacro

%macro  align_cmp_jne 3

  %%i: 
    cmp %1, %2
    jne %3
  %%endi: 
        times 10-(%%endi-%%i) nop 

%endmacro

_start:
    align_nops mov dword [r15], "Pass"
    align_nops mov dword [r15+4], "word"
    align_nops mov dword [r15+8], " :\n"

    ; Write to stdout
    align_nops mov       rax, 1              
    align_nops mov       rdi, 1              
    align_nops mov       rsi, r15                  
    align_nops mov       rdx, 10             
    align_nops syscall

    ; Read from stdin
    align_nops mov       rax, 0
    align_nops mov       rdi, 0
    align_nops mov       rsi, r15
    align_nops mov       rdx, 16
    align_nops syscall

    align_nops xor dword [r15], 0xd5702daa
    align_nops xor dword [r15+2], 0xf2d4ad0c
    align_nops xor dword [r15+4], 0xb95834c9
    align_nops xor dword [r15+6], 0x17ae510d
    align_nops xor dword [r15+8], 0xb8dfefa0
    align_nops xor dword [r15+10], 0xb7bd13d1
    align_nops xor dword [r15+12], 0x87e8f006

    align_nops mov dword [r15+16], 0x27041cc7
    align_nops mov dword [r15+20], 0xba0aab4b
    align_nops mov dword [r15+24], 0xf462993d
    align_nops mov dword [r15+28], 0xf49d17d8

    ; compare 
    align_nops mov eax, [r15]
    align_nops mov edi, [r15+16]
    align_cmp_jne eax, edi, fail

    align_nops mov eax, [r15+4]
    align_nops mov edi, [r15+20]
    align_cmp_jne eax, edi, fail

    align_nops mov eax, [r15+8]
    align_nops mov edi, [r15+24]
    align_cmp_jne eax, edi, fail

    align_nops mov eax, [r15+12]
    align_nops mov edi, [r15+28]
    align_cmp_jne eax, edi, fail

    align_nops jmp success

success:
    align_nops mov dword [r15], "Nice"
    align_nops mov dword [r15+4], " !!"
    align_nops mov rdx, 7 ; str len
    align_nops mov r14, 0
    align_nops jmp end

fail:
    align_nops mov dword [r15], "nop "
    align_nops mov dword [r15+4], "^^"
    align_nops mov rdx, 6 ; str len
    align_nops mov r14, 1
    align_nops jmp end

end:
    align_nops mov       rax, 1              
    align_nops mov       rdi, 1              
    align_nops mov       rsi, r15                  
    ;align_nops mov       rdx, 10             
    align_nops syscall

    align_nops mov       rax, 60
    align_nops mov       rdi, r14
    align_nops syscall