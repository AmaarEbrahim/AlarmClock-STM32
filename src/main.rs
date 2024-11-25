#![no_std]
#![no_main]

use core::panic::PanicInfo;

// #[link(name = "startup")]
extern "C" {
    fn startup();
}

union Vector {
    integer: u32,
    vector: fn() -> ()
}

fn ResetVector() {
    unsafe {
        startup();
        main();
        }
}

fn Hang() {
    loop {}
}

#[link_section = ".vector_core"]
#[no_mangle]
pub static vtable: [Vector; 15] = [
    Vector {vector: ResetVector},   // Reset
    Vector {vector: Hang},          // NMI_Handler
    Vector {vector: Hang},          // HardFault_Handler
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {integer: 0},             // Reserved
    Vector {vector: Hang},          // SVC_Handler
    Vector {integer: 0},            // Reserved
    Vector {integer: 0},            // Reserved
    Vector {vector: Hang},    // PendSV_Handler
    Vector {vector: Hang},    // SysTick_Handler
];

#[no_mangle]
fn main() -> () {

    // blink an LED. Try doing it in C first.
    // https://blog.embeddedexpert.io/?p=837
    // Submodule CMSIS-core for stm32l0 devices: https://github.com/STMicroelectronics/cmsis-device-l0/tree/master

    loop {
    }
}

#[panic_handler]
fn panic(_a: &PanicInfo) -> ! {
    loop {}
}
