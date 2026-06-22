/* Cortex-M0 harness for the MQTT-C deserialiser.
 *
 * The fuzz input lives at 0x2000_4000. The harness parses an MQTT fixed
 * header, and when the packet is a PUBLISH it deserialises it and then does
 * what a typical application would: copy the application message into a
 * scratch buffer. The scratch buffer and the input buffer are surrounded by
 * unmapped guard holes, so any out-of-bounds access caused by a malformed
 * length becomes a detectable guard hit.
 *
 * Writing the halt register ends the run cleanly.
 */
#include <stdint.h>
#include <stddef.h>

#include "mqtt.h"

#define INPUT_BASE 0x20004000u
#define SCRATCH_BASE 0x20002000u
#define HALT_REG (*(volatile uint32_t *)0x4000f000u)

static struct mqtt_response g_response;

static void *h_memcpy(void *dst, const void *src, size_t n)
{
    uint8_t *d = (uint8_t *)dst;
    const uint8_t *s = (const uint8_t *)src;
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    return dst;
}

int main(void)
{
    const uint8_t *buf = (const uint8_t *)INPUT_BASE;

    ssize_t rv = mqtt_unpack_fixed_header(&g_response, buf, 0x1000);
    if (rv < 0) {
        HALT_REG = 0;
        for (;;) {
        }
    }
    buf += rv;

    if (g_response.fixed_header.control_type == MQTT_CONTROL_PUBLISH) {
        rv = mqtt_unpack_publish_response(&g_response, buf);
        if (rv >= 0) {
            /* What a consumer does with the payload. */
            h_memcpy((void *)SCRATCH_BASE,
                     g_response.decoded.publish.application_message,
                     g_response.decoded.publish.application_message_size);
        }
    }

    HALT_REG = 0;
    for (;;) {
    }
}
