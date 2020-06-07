#include <stddef.h>

void js_snap_init();

struct JSSnapInstance* js_snap_instance_from_snapshot(
  const void* data,
  size_t data_length,
  const char* export_name);

struct JSSnapInstance* js_snap_instance_from_source(
  const char* source,
  const char* export_name);

void js_snap_instance_delete(struct JSSnapInstance*);

const char* js_snap_instance_call(
  struct JSSnapInstance*,
  const char* name,
  const char* params,
  const char** result_ptr,
  int* result_len);
