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

5. Can connect to microcontroller through OpenOCD and gdb-multiarch
    - Install OpenOCD and gdb-multiarch
    - Put OpenOCD command in a file (openocd.sh) to automate running it
    - Put GDB commands in a file (gdb_start) to automate command execution
    - Run `./openocd.sh` to activate OpenOCD
    - Run `gdb-multiarch -x gdb_start` to connect to OpenOCD's gdb server then
      run the commands

6. Blinked an LED in C
    - build.rs builds the C code with "-mthumb" and "-mcpu=cortexm0plus" flags
        this ensures that the compiler translates code into assembly instructions
        supported by the cortex-m0+? before adding these flags, the processor 
        encountered a hardfault when this line of code was executed:

        ```asm
            8000154:	f7ff ff7c 	blx	8000050 <startup>
        ```
        blx is "branch and link with exchange". It exchanges the instruction set
        from Thumb to ARM. Since the cortex-m0+ doesn't only supports Thumb, running
        this line of code causes a hard fault.

        With the above flags added, bl is used instead of blx.

    - Created functions based on code found online (https://blog.embeddedexpert.io/?p=837) to blink an LED in C
    - Removed "-fPIC" flag from build.rs because it was preventing linker symbols
    from having the right values in the executable. When fPIC was there, the processor
    would encounter a hardfault because there were invalid memory accesses.
    - learned some GDB commands to help debug: step, stepi n, finish, continue
    - submoduled the STM32CubeL0 MCU Firmware Package (https://github.com/STMicroelectronics/STM32CubeL0/tree/master)
        It provides CMSIS-Core, CMSIS-Device, and a bunch of other libraries
        Also, I had to change build.rs to search for header files in the Core and Device subdirectories so I could use stm32l073xx.h header file for its definitions.



# Useful things to remember

rustflags: flags passed to rustc that are defined in a .cargo/config.toml

rust-lld: the linker used by rust. It is a wrapper for lld.

link-args: flags passed to the linker. These are defined in rustflags

Flags are passed to the linker indirectly by gcc with -Wl,

Specify a linker script with -T

Use `ar rcs [output file name] [input file name]` to archive an object file

Install gdb-multiarch to use gdb on different platforms.

# Problems
build.rs specifies to recompile startup.c when it changes. However, startup.c 
seems to only get compiled when its corresponding archive and object files have 
been deleted. If startup.c is changed, but the old object and archive files aren't
deleted, startup.c won't get compiled. 

Build times are really long even though the project is small. Maybe it is because of the submodule I added. Does rustc search through all directories in the project to find rust files?

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
