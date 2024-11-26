#include "stm32l073xx.h"
#include <stdint.h>

extern uint32_t _etext, _data, _edata, _bstart, _bend;

void startup(void)
{

    uint32_t *src = &_etext;
    uint32_t *dst = &_data;

    /* ROM has data at end of text; copy it.  */
    while (dst < &_edata)
        *dst++ = *src++;

    /* Zero bss.  */
    for (dst = &_bstart; dst < &_bend; dst++)
        *dst = 0;
}

void configure_led()
{
    RCC->IOPENR |= RCC_IOPENR_GPIOAEN;
    /*Set PA5 as output pin*/
    GPIOA->MODER |= GPIO_MODER_MODE5_0;
    GPIOA->MODER &= ~GPIO_MODER_MODE5_1;
}

void wait()
{
    for (uint32_t i = 0; i < 100000; i++)
        ;
}

void turn_led_on()
{
    GPIOA->ODR |= GPIO_ODR_OD5;
}

void turn_led_off()
{
    GPIOA->ODR &= ~GPIO_ODR_OD5;
}