use std::process::Command;

fn main() {

    const core_path: &str = "./STM32CubeL0/Drivers/CMSIS/Core/Include";
    const device_path: &str = "./STM32CubeL0/Drivers/CMSIS/Device/ST/STM32L0xx/Include";
    // 
    // -fPIC: this flag at the very least makes the executable not have linker symbols created with "." in the right place memory  
    Command::new("arm-none-eabi-gcc").args(&["./src/startup.c", "-c", "-mthumb", "-mcpu=cortex-m0plus", "-o", "./startup.o", "-I", core_path, "-I", device_path, "-ggdb"]).status().unwrap();
    Command::new("arm-none-eabi-ar").args(&["rcus", "./libstartup.a", "./startup.o"]).status().unwrap();

    println!("cargo::rustc-link-search=native=./");
    println!("cargo::rustc-link-lib=static=startup");
    println!("cargo::rerun-if-changed=./src/startup.c");

}