openocd -f interface/stlink.cfg -c "transport select hla_swd" \
        -f target/stm32l0_dual_bank.cfg