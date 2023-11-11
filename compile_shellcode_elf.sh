#!/bin/bash
nasm -f elf64 shellcode.asm -o shellcode.o
ld -o shellcode shellcode.o
rm shellcode.o