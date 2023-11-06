use std::{
    alloc::{alloc, Layout},
    arch::asm,
};

use libc::{mprotect, PROT_EXEC, PROT_READ, PROT_WRITE, _SC_PAGESIZE};

fn main() {
    unsafe {
        let page_size = page_size::get();
        let ptr = alloc(Layout::from_size_align(50, page_size).unwrap());
        mprotect(
            ptr.cast(),
            _SC_PAGESIZE as _,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );

        let slice = core::slice::from_raw_parts_mut(ptr, 50);
        slice[0..8].copy_from_slice(&[0x48, 0xC7, 0xC0, 0x05, 0x00, 0x00, 0x00, 0xC3]);

        let mut rax = 0;

        asm!(
            "call {0}",
            in(reg) ptr,
            inout("rax") rax,
        );
        println!("Hello, world! rax = {}", rax);
    }
}
