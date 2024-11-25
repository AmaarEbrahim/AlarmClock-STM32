extern int _etext, _data, _edata, _bstart, _bend;

void startup(void)
{

    int *src = &_etext;
    int *dst = &_data;

    /* ROM has data at end of text; copy it.  */
    while (dst < &_edata)
        *dst++ = *src++;

    /* Zero bss.  */
    for (dst = &_bstart; dst < &_bend; dst++)
        *dst = 0;
}