EXE_PATH=../target/thumbv6m-none-eabi/debug/alarm-clock-rs

arm-none-eabi-objdump -d -S $EXE_PATH > ../data/alarm-clock-rs-dis.s