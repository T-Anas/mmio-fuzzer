/* Cortex-M0 harness for the Eclipse Paho MQTTPacket deserialisers.
 *
 * The fuzz input lives at 0x2000_4000. The harness dispatches on the MQTT
 * packet type and runs the matching deserialiser, then halts. The input
 * region is followed by a poisoned redzone and then unmapped guard holes, so
 * an out-of-bounds read inside the library is reported by the emulator.
 */
#include <stdint.h>
#include <stddef.h>

#include "MQTTPacket.h"
#include "MQTTConnect.h"
#include "MQTTSubscribe.h"
#include "MQTTUnsubscribe.h"

#define INPUT_BASE 0x20004000u
#define INPUT_LEN 0x1000
#define HALT_REG (*(volatile uint32_t *)0x4000f000u)

static MQTTString g_topics[8];
static int g_qos[8];

int main(void)
{
    unsigned char *buf = (unsigned char *)INPUT_BASE;
    int len = INPUT_LEN;
    unsigned char type = (unsigned char)(buf[0] >> 4);

    switch (type) {
    case 2: { /* CONNACK */
        unsigned char sp = 0, rc = 0;
        MQTTDeserialize_connack(&sp, &rc, buf, len);
        break;
    }
    case 3: { /* PUBLISH */
        unsigned char dup = 0, retained = 0;
        int qos = 0, payloadlen = 0;
        unsigned short packetid = 0;
        MQTTString topic;
        unsigned char *payload = 0;
        if (MQTTDeserialize_publish(&dup, &qos, &retained, &packetid, &topic,
                                    &payload, &payloadlen, buf, len) == 1) {
            volatile int sum = 0;
            int n = payloadlen < 64 ? payloadlen : 64;
            for (int i = 0; i < n; i++) {
                sum += payload[i];
            }
        }
        break;
    }
    case 4: case 5: case 6: case 7: { /* PUBACK/REC/REL/COMP */
        unsigned char packettype = 0, dup = 0;
        unsigned short packetid = 0;
        MQTTDeserialize_ack(&packettype, &dup, &packetid, buf, len);
        break;
    }
    case 8: { /* SUBSCRIBE */
        unsigned char dup = 0;
        unsigned short packetid = 0;
        int count = 0;
        MQTTDeserialize_subscribe(&dup, &packetid, 8, &count, g_topics, g_qos, buf, len);
        break;
    }
    case 9: { /* SUBACK */
        unsigned short packetid = 0;
        int count = 0, granted[8];
        MQTTDeserialize_suback(&packetid, 8, &count, granted, buf, len);
        break;
    }
    case 10: { /* UNSUBSCRIBE */
        unsigned char dup = 0;
        unsigned short packetid = 0;
        int count = 0;
        MQTTDeserialize_unsubscribe(&dup, &packetid, 8, &count, g_topics, buf, len);
        break;
    }
    case 11: { /* UNSUBACK */
        unsigned short packetid = 0;
        MQTTDeserialize_unsuback(&packetid, buf, len);
        break;
    }
    default:
        break;
    }

    HALT_REG = 0;
    for (;;) {
    }
}
