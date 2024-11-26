EXE_PATH=../target/thumbv6m-none-eabi/debug/alarm-clock-rs

# display symbols
arm-none-eabi-readelf --symbols $EXE_PATH > ../data/alarm-clock-rs-readelf

# display sections
arm-none-eabi-readelf --sections $EXE_PATH >> ../data/alarm-clock-rs-readelf

# display segments
arm-none-eabi-readelf --segments $EXE_PATH >> ../data/alarm-clock-rs-readelf

# hexdump
hexdump -C $EXE_PATH > ../data/alarm-clock-rs-hexdump