/*
 * Host AddressSanitizer reproducer for the off-by-one write in
 * MQTTFormat_toServerString.
 *
 * Build (from this directory):
 *   gcc -g -fsanitize=address -DMQTT_SERVER -DMQTT_CLIENT -I ../mqttpacket \
 *       poc_format_offbyone.c ../mqttpacket/MQTTPacket.c \
 *       ../mqttpacket/MQTTFormat.c ../mqttpacket/MQTTDeserializePublish.c \
 *       ../mqttpacket/MQTTConnectClient.c ../mqttpacket/MQTTConnectServer.c \
 *       ../mqttpacket/MQTTSubscribeServer.c ../mqttpacket/MQTTSubscribeClient.c \
 *       ../mqttpacket/MQTTUnsubscribeServer.c ../mqttpacket/MQTTUnsubscribeClient.c \
 *       -o poc_format_offbyone
 *   ./poc_format_offbyone
 *
 * Expected: AddressSanitizer reports a 1-byte stack-buffer-overflow WRITE in
 * MQTTFormat_toServerString at MQTTFormat.c:265, on a perfectly valid packet.
 */
#include <stdio.h>

#include "MQTTFormat.h"

int main(void)
{
    unsigned char pingresp[2] = {0xD0, 0x00};
    char out[16];
    MQTTFormat_toServerString(out, (int)sizeof(out), pingresp, (int)sizeof(pingresp));
    printf("returned\n");
    return 0;
}
