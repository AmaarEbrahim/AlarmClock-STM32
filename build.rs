use std::process::Command;

fn main() {

    Command::new("arm-none-eabi-gcc").args(&["./src/startup.c", "-c", "-fPIC", "-o", "./target/startup.o"]).status().unwrap();
    Command::new("arm-none-eabi-ar").args(&["rcus", "./target/libstartup.a", "./target/startup.o"]).status().unwrap();

    println!("cargo::rustc-link-search=native=./target");
    println!("cargo::rustc-link-lib=static=startup");
    println!("cargo::rerun-if-changed=./src/startup.c");

}