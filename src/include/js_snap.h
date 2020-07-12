#ifndef JS_SNAP_H_
#define JS_SNAP_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct JSSnapInstance JSSnapInstance;

void js_snap_init(void);

void js_snap_instance_call(JSSnapInstance *instance,
                           const char *name,
                           const char *params,
                           const char **result_ptr,
                           int32_t *result_len);

void js_snap_instance_delete(JSSnapInstance *instance);

JSSnapInstance *js_snap_instance_from_bundle(const char *export_name);

JSSnapInstance *js_snap_instance_from_snapshot(const uint8_t *data,
                                               uintptr_t data_length,
                                               const char *export_name);

JSSnapInstance *js_snap_instance_from_source(const char *source, const char *export_name);

#endif /* JS_SNAP_H_ */
