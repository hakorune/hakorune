// Shared compat-only Hako forward hook registry surface.
//
// Keep future spawn / string dispatch hook storage and C ABI registration here
// so nyash_kernel and the libc shim do not each own duplicate registry behavior.

typedef int64_t (*nyrt_hako_future_spawn_instance_fn)(
    int64_t a0, int64_t a1, int64_t a2, int64_t argc);
typedef int64_t (*nyrt_hako_string_dispatch_fn)(
    int64_t op, int64_t a0, int64_t a1, int64_t a2);

static nyrt_hako_future_spawn_instance_fn g_hako_future_spawn_instance = 0;
static nyrt_hako_string_dispatch_fn g_hako_string_dispatch = 0;

int64_t nyrt_hako_register_future_spawn_instance(nyrt_hako_future_spawn_instance_fn f) {
  g_hako_future_spawn_instance = f;
  return 1;
}

int64_t nyrt_hako_register_string_dispatch(nyrt_hako_string_dispatch_fn f) {
  g_hako_string_dispatch = f;
  return 1;
}

int nyrt_hako_try_future_spawn_instance(
    int64_t a0, int64_t a1, int64_t a2, int64_t argc, int64_t* out_value) {
  if (!g_hako_future_spawn_instance || !out_value) return 0;
  *out_value = g_hako_future_spawn_instance(a0, a1, a2, argc);
  return 1;
}

int nyrt_hako_try_string_dispatch(
    int64_t op, int64_t a0, int64_t a1, int64_t a2, int64_t* out_value) {
  if (!g_hako_string_dispatch || !out_value) return 0;
  *out_value = g_hako_string_dispatch(op, a0, a1, a2);
  return 1;
}
