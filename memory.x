/* if you change the linker script, run `cargo clean` to clear the cache */

MEMORY {
    sram (wrx) : ORIGIN = 0x20000000, LENGTH = 20K
    flash (rx) : ORIGIN = 0x08000000, LENGTH = 192K 
}



SECTIONS {
    .stack_ptr : {
        _stack_start = ORIGIN(sram) + LENGTH(sram);
        LONG(_stack_start)
    } > flash
    .vector_core : {
        KEEP(*(.vector_core))
    } > flash
    .text : {
        KEEP(*(.text))
        KEEP(*(.text.*))
        _etext = .;
    } > flash

    .data : {
        _data = .;
        KEEP(*(.data))
        KEEP(*(.data.*))
        _edata = .;
    } > sram AT> flash

    .bss : {
        _bstart = .;
        KEEP(*(.bss))
        KEEP(*(.bss.*))
        _bend = .;
    } > sram

    
}

