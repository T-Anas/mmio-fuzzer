/*
 * uart_demo: a small, deliberately buggy Cortex-M0 firmware.
 *
 * It models a device that is only usable once a status bit reads set, then
 * consumes a command word from MMIO. If the top bit of the command is set it
 * uses the low half as an offset into a bogus base address and writes there.
 * On a real part that is an imprecise bus fault; in our emulator it is an
 * unmapped access. Either way it is a target bug worth reproducing.
 */

#include <stdint.h>

#define UART_BASE   0x40000000u
#define UART_CTRL   (*(volatile uint32_t *)(UART_BASE + 0x00))
#define UART_STATUS (*(volatile uint32_t *)(UART_BASE + 0x04))
#define UART_DATA   (*(volatile uint32_t *)(UART_BASE + 0x08))
#define UART_CMD    (*(volatile uint32_t *)(UART_BASE + 0x0c))

#define STATUS_READY 0x1u

int main(void)
{
    /* Wait for the device to come up. Without a proper model this spins. */
    while ((UART_STATUS & STATUS_READY) == 0) {
    }

    UART_CTRL = 0x00000003u;

    uint32_t cmd = UART_CMD;
    if (cmd & 0x80000000u) {
        /* Seeded bug: attacker-controlled address, no validation. */
        volatile uint32_t *ptr = (volatile uint32_t *)(0x80000000u | (cmd & 0xffffu));
        *ptr = 0xdeadbeefu;
    }

    UART_DATA = cmd & 0xffu;

    for (;;) {
    }
}
