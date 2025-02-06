STACK_SIZE = 0x400;
SYS_HEAP_SIZE = 0x1000;

MEMORY {
    ROM (rx) : ORIGIN = 0x00000000, LENGTH = 0x00008000
    RAM (rw) : ORIGIN = 0x20000000, LENGTH = 0x00004000
}


ENTRY(_start)
SECTIONS {
    . = ORIGIN(ROM);

    #if defined(BOOTROM)
    .boot : {
        LONG(Image$$STACK$$ZI$$Limit)
        LONG(_start)
    } > ROM
    #endif /*BOOTROM*/

    .text : {
        *(.text*)
        *(.rodata*)
        _etext = .;
    } > ROM

    #if defined(RAM_LINK)
    . = ORIGIN(RAM);
    #else /*ROM_LINK*/
    . = ALIGN(32);
    #endif /*RAM_LINK | ROM_LINK*/

    Load$$DATA$$Base = .;
    .data : AT(Load$$DATA$$Base) {
        Image$$DATA$$Base = .;

        *(.data*)
    } > RAM

    .bss (NOLOAD) : {
        Image$$BSS$$ZI$$Base = .;
    
        *(.bss*)
        *(COMMON)
    
        . = ALIGN(32);
        Image$$BSS$$ZI$$Limit = .;
    } > RAM

    .heap (NOLOAD) : {
        Image$$HEAP$$ZI$$Base = Image$$BSS$$ZI$$Limit;

        #if !defined(SYS_HEAP_SIZE)
        #define SYS_HEAP_SIZE 0
        #endif /*SYS_HEAP_SIZE*/
        
        #if (0 == SYS_HEAP_SIZE)
        Image$$HEAP$$ZI$$Limit = ORIGIN(RAM) + LENGTH(RAM) - STACK_SIZE;
        #else /*SYS_HEAP_SIZE*/
        Image$$HEAP$$ZI$$Limit = Image$$HEAP$$ZI$$Base + SYS_HEAP_SIZE;
        #endif/*SYS_HEAP_SIZE*/

        /* . = Image$$HEAP$$ZI$$Limit; */
    } > RAM

    .stack (NOLOAD) : {
        /* . = ALIGN(32);
        Image$$STACK$$Base = .;
        . = . + STACK_SIZE; */
        Image$$STACK$$ZI$$Limit = ORIGIN(RAM) + LENGTH(RAM);
    } > RAM
}
