.syntax unified
.thumb

.section .vectors, "a", %progbits
.global _start
.thumb_func
_start:
    .word 0x20001000        @ initial stack pointer
    .word reset_handler     @ reset vector

.section .text
.global reset_handler
.thumb_func
reset_handler:
    bl main
1:  b 1b
