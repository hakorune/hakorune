#ifndef NYRT_H
#define NYRT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Phase 20.36/20.37: C‑ABI scaffold (PoC)
// Minimal kernel API for Rust runtime; subject to evolution.

// Initialize NyRT kernel. Returns 0 on success.
int nyrt_init(void);

// Tear down NyRT kernel.
void nyrt_teardown(void);

// Load MIR(JSON v1) text. Returns a handle (>=1) or 0 on error.
uint64_t nyrt_load_mir_json(const char* json_text);

// Execute main() of the loaded module. Returns process‑like rc (0..255).
int nyrt_exec_main(uint64_t module_handle);

// Verifier gate: validate MIR(JSON v1) contract before execution.
// Returns 0 on success.
int nyrt_verify_mir_json(const char* json_text);

// Safety gate: validate lifecycle/unsafe boundary before execution.
// Returns 0 on success.
int nyrt_safety_check_mir_json(const char* json_text);

// Hostcall bridge (name = "env.*" / provider). Returns 0 on success.
int nyrt_hostcall(const char* name, const char* method,
                  const char* payload_json,
                  /*out*/ char* out_buf, unsigned int out_buf_len);

// Handle lifecycle extensions (Phase 29y ABI shim).
// Contract SSOT:
// - retain_h(0) -> 0
// - release_h(0) -> no-op
int64_t nyrt_handle_retain_h(int64_t handle);
void nyrt_handle_release_h(int64_t handle);

// Hako forward hook registry (Phase 29cc HFK-min2).
// Core C ABI canonical surface; dot-name aliases remain compatibility exports.
typedef int64_t (*nyrt_hako_plugin_invoke_by_name_fn)(
    int64_t recv_handle, const char* method, int64_t argc, int64_t a1, int64_t a2);
typedef int64_t (*nyrt_hako_future_spawn_instance_fn)(
    int64_t a0, int64_t a1, int64_t a2, int64_t argc);
typedef int64_t (*nyrt_hako_string_dispatch_fn)(
    int64_t op, int64_t a0, int64_t a1, int64_t a2);

// Register hook function pointers.
// Passing NULL resets/unregisters the hook.
int64_t nyrt_hako_register_plugin_invoke_by_name(nyrt_hako_plugin_invoke_by_name_fn f);
int64_t nyrt_hako_register_future_spawn_instance(nyrt_hako_future_spawn_instance_fn f);
int64_t nyrt_hako_register_string_dispatch(nyrt_hako_string_dispatch_fn f);

// Try dispatching to a registered hook.
// Returns 1 when dispatched and writes result to out_value; returns 0 when no hook.
int nyrt_hako_try_plugin_invoke_by_name(
    int64_t recv_handle, const char* method, int64_t argc, int64_t a1, int64_t a2, int64_t* out_value);
int nyrt_hako_try_future_spawn_instance(
    int64_t a0, int64_t a1, int64_t a2, int64_t argc, int64_t* out_value);
int nyrt_hako_try_string_dispatch(
    int64_t op, int64_t a0, int64_t a1, int64_t a2, int64_t* out_value);

// Runtime V0 helper slice (`string_len`, `array_get_i64`, `array_set_i64`) is
// currently exported as plugin-style symbols (`nyash.*`) and routed from
// `.hako` VM entry boxes. See docs/reference/abi/nyrt_c_abi_v0.md.

#ifdef __cplusplus
}
#endif

#endif // NYRT_H
