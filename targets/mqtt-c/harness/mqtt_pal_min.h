/*
 * Minimal MQTT-C platform layer for the mmio-fuzz Cortex-M0 harness.
 *
 * MQTT-C normally ships a POSIX/Windows platform file. On a bare-metal target
 * we do not have sockets or threads, and we only exercise the deserialisation
 * code, so most of the platform layer is never reached. This header provides
 * just enough surface for mqtt.c to compile; unused functions are dropped by
 * `--gc-sections` at link time.
 */
#ifndef MQTT_PAL_MIN_H
#define MQTT_PAL_MIN_H

#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>
#include <limits.h>

typedef int mqtt_pal_socket_handle;
typedef int ssize_t;
typedef unsigned long mqtt_pal_time_t;
typedef int mqtt_pal_mutex_t;

/* Freestanding string/memory declarations. The retained deserialiser paths do
 * not call these, but mqtt.c references them in code that is dropped at link
 * time, and implicit declarations would change their ABI. */
void *memcpy(void *dst, const void *src, size_t n);
void *memmove(void *dst, const void *src, size_t n);
void *memset(void *dst, int c, size_t n);
size_t strlen(const char *s);

#define MQTT_PAL_HTONS(s) \
    ((uint16_t)((((uint16_t)(s) & 0x00ffu) << 8) | (((uint16_t)(s) & 0xff00u) >> 8)))
#define MQTT_PAL_NTOHS(s) MQTT_PAL_HTONS(s)
#define MQTT_PAL_TIME() ((mqtt_pal_time_t)0)

#define MQTT_PAL_MUTEX_INIT(m) ((void)(m))
#define MQTT_PAL_MUTEX_LOCK(m) ((void)(m))
#define MQTT_PAL_MUTEX_UNLOCK(m) ((void)(m))

/* Prototypes kept for compilation; the harness never links them. */
ssize_t mqtt_pal_sendall(mqtt_pal_socket_handle fd, const void *buf, size_t len, int flags);
ssize_t mqtt_pal_recvall(mqtt_pal_socket_handle fd, void *buf, size_t bufsz, int flags);

#endif /* MQTT_PAL_MIN_H */
