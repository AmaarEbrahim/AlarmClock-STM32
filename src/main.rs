#![no_std]
#![no_main]

use core::panic::PanicInfo;

// #[link(name = "startup")]
extern "C" {
    fn startup();
}

extern "C" {
    fn configure_led();
    fn wait();
    fn turn_led_on();
    fn turn_led_off();
}

union Vector {
    integer: u32,
    vector: fn() -> ()
}

fn stuff() -> u32 {
    return 3;
}

#[no_mangle]
fn ResetVector() {
    unsafe {
        startup();
        main();
        }

    
}

#[no_mangle]
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
    // Submodule CMSIS-device for stm32l0 devices: https://github.com/STMicroelectronics/cmsis-device-l0/tree/master
    // https://github.com/STMicroelectronics/STM32CubeL0

    let h: u32 = stuff();

    unsafe {

    configure_led();

    loop {
        turn_led_on();
        wait();
        turn_led_off();
        wait();
    }

}
}

#[panic_handler]
#[no_mangle]
fn panic(_a: &PanicInfo) -> ! {
    loop {}
}
