# Overview
Alarm clock based on an STM32L073RZ microcontroller.

Firmware written in Rust. I am not using the Cortex-M0 rust library
because I want to write the firmware mostly from scratch.

# Progress

Just a collection of things I got working.

1. Got Rust to cross-compile for Cortex M0+
    - Had to install thumbv6m-none-eabi package
    - Set target in .cargo/config.toml
    - Set #![no_main], #![no_std], #[panic_handler], and #[no_mangle]
    - Linker script where KEEP was used on the .text and .text.* sections

2. Analyzed ELF files with readelf and hexdump
    - Ran arm-none-eabi-readelf with flags for sections, segments, symbols
    - Ran hexdump -C

2. Used build script (build.rs) to compile and link C code
    - Used Command class to compile and archive a file with C code
    - Used println! to instruct rustc to link the archive to Rust code
    - Used extern C to access functions from the static library in Rust

3. Created a vector table to hold ISRs
    - Created a union of either a u32 or fn() -> ()
    - Created link section for vector table called .vector_core
    - Added segment called .vector_core to linker script 
    - Vectors are the Cortex M0+ system vectors from the reference manual

4. Added a variable to linker script to store where the start of the stack would be
    - Used LONG(x) function
    - Created segment just for stack pointer

# Useful things to remember

rustflags: flags passed to rustc that are defined in a .cargo/config.toml

rust-lld: the linker used by rust. It is a wrapper for lld.

link-args: flags passed to the linker. These are defined in rustflags

Flags are passed to the linker indirectly by gcc with -Wl,

Specify a linker script with -T

Use `ar rcs [output file name] [input file name]` to archive an object file

# Problems
build.rs specifies to recompile startup.c when it changes. However, startup.c 
seems to only get compiled when its corresponding archive and object files have 
been deleted. If startup.c is changed, but the old object and archive files aren't
deleted, startup.c won't get compiled. 

# Resources:

1. ld (also talks about Linker Command Language): https://sourceware.org/binutils/docs/ld/index.html#SEC_Contents
2. gcc 10.3.1: https://gcc.gnu.org/onlinedocs/gcc-10.3.0/gcc/index.html#SEC_Contents
3. arm-none-eabi utilities: https://manpages.debian.org/testing/binutils-arm-none-eabi/index.html
    - doesn't talk about arm-none-eabi-gcc. arm-none-eabi-gcc is actually covered by the gcc docs
4. Learning Rust: https://doc.rust-lang.org/book/title-page.html
5. Rust language reference: https://doc.rust-lang.org/reference/introduction.html
6. rustc (compiler): https://doc.rust-lang.org/rustc/what-is-rustc.html
7. cargo (manages rust projects): https://doc.rust-lang.org/cargo/index.html
8. rustup (manages versions of rust): https://rust-lang.github.io/rustup/index.html

https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program
