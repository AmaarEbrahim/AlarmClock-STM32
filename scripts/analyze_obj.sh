OBJ_PATH="../target/thumbv6m-none-eabi/debug/deps/alarm_clock_rs-cd0f6c543205a683.o"
# display symbols
arm-none-eabi-readelf --symbols $OBJ_PATH

# display sections
arm-none-eabi-readelf --sections $OBJ_PATH

# display segments
arm-none-eabi-readelf --segments $OBJ_PATH

# hexdump
# hexdump -C $OBJ_PATH