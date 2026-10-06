.syntax unified
.thumb

.section .vectors, "a", %progbits
.global _start
.thumb_func
_start:
    .word 0x20000800        @ initial stack pointer (top of stack region)
    .word reset_handler     @ reset vector

.section .text
.global reset_handler
.thumb_func
reset_handler:
    @ r0 = input buffer base (0x20004000)
    ldr r0, =0x20004000
    @ read one byte well past the 4 KiB input region: this lands in the guard
    ldr r1, =0x2000
    ldrb r2, [r0, r1]
    @ if we somehow survive, hold the value and spin
1:  b 1b

.align 2
.pool
