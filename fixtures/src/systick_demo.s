.syntax unified
.thumb

.section .vectors, "a", %progbits
.global _start
.thumb_func
_start:
    .word 0x20000800        @ initial stack pointer
    .word reset_handler     @ 1  reset
    .word 0                 @ 2  NMI
    .word 0                 @ 3  HardFault
    .word 0                 @ 4
    .word 0                 @ 5
    .word 0                 @ 6
    .word 0                 @ 7
    .word 0                 @ 8
    .word 0                 @ 9
    .word 0                 @ 10
    .word 0                 @ 11 SVCall
    .word 0                 @ 12
    .word 0                 @ 13
    .word 0                 @ 14 PendSV
    .word systick_handler   @ 15 SysTick

.section .text
.global reset_handler
.thumb_func
reset_handler:
    @ configure SysTick: load 50, value 50, control = enable | tickint | clksource
    ldr r0, =0xE000E014
    ldr r1, =50
    str r1, [r0]
    ldr r0, =0xE000E018
    str r1, [r0]
    ldr r0, =0xE000E010
    movs r1, #7
    str r1, [r0]
1:  b 1b

@ Runs on every SysTick. Counts to three, then halts the harness.
.global systick_handler
.thumb_func
systick_handler:
    ldr r0, =0x20000000
    ldr r1, [r0]
    adds r1, #1
    str r1, [r0]
    cmp r1, #3
    blt 2f
    ldr r2, =0x4000F000
    movs r3, #0
    str r3, [r2]
2:  bx lr

.align 2
.pool
