/* Cortex-M0 harness for the MQTT-C deserialiser.
 *
 * The fuzz input lives at 0x2000_4000. The harness runs the full response
 * deserialiser (`mqtt_unpack_response`, which dispatches on the packet type)
 * and then plays the part of a typical application: it copies a PUBLISH
 * payload, or walks a SUBACK's return codes. The scratch buffer and the input
 * buffer are surrounded by unmapped guard holes, so any out-of-bounds access
 * caused by a malformed length becomes a detectable guard hit.
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
static volatile uint32_t g_sink;

static void *h_memcpy(void *dst, const void *src, size_t n)
{
    uint8_t *d = (uint8_t *)dst;
    const uint8_t *s = (const uint8_t *)src;
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
    return dst;
}

/* What an application does with a delivered PUBLISH. */
static void consume_publish(struct mqtt_response *r)
{
    h_memcpy((void *)SCRATCH_BASE,
             r->decoded.publish.application_message,
             r->decoded.publish.application_message_size);
}

/* What an application does with a SUBACK. */
static void consume_suback(struct mqtt_response *r)
{
    const uint8_t *codes = (const uint8_t *)r->decoded.suback.return_codes;
    uint32_t acc = 0;
    for (size_t i = 0; i < r->decoded.suback.num_return_codes; i++) {
        acc += codes[i];
    }
    g_sink = acc;
}

int main(void)
{
    const uint8_t *buf = (const uint8_t *)INPUT_BASE;

    ssize_t rv = mqtt_unpack_response(&g_response, buf, 0x1000);
    if (rv >= 0) {
        switch (g_response.fixed_header.control_type) {
        case MQTT_CONTROL_PUBLISH:
            consume_publish(&g_response);
            break;
        case MQTT_CONTROL_SUBACK:
            consume_suback(&g_response);
            break;
        default:
            break;
        }
    }

    HALT_REG = 0;
    for (;;) {
    }
}
